use crate::{
    job,
    progress::{ProgressState, ProgressUpdate},
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashMap, error::Error, process::Stdio, str::FromStr, sync::Arc, time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::RwLock,
};

static DURATION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*Duration: ([0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{2,6})").unwrap());

#[derive(Debug, Clone, Copy)]
struct FFMpegDuration(Duration);

#[derive(Debug, PartialEq, Eq)]
struct FFMpegDurationParseError;

impl FromStr for FFMpegDuration {
    type Err = FFMpegDurationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ':');

        let hours = parts
            .next()
            .and_then(|x| x.parse::<u64>().ok())
            .ok_or(FFMpegDurationParseError)?;
        let minutes = parts
            .next()
            .and_then(|x| x.parse::<u64>().ok())
            .ok_or(FFMpegDurationParseError)?;

        let seconds_and_milliseconds = parts.next().ok_or(FFMpegDurationParseError)?;
        let (seconds, milliseconds_parts) = seconds_and_milliseconds
            .split_once('.')
            .and_then(|(s, m)| match (s.parse::<u64>(), m.parse::<u64>()) {
                (Ok(s), Ok(m)) => Some((s, m)),
                _ => None,
            })
            .ok_or(FFMpegDurationParseError)?;

        let mut seconds_sum = hours * 60 * 60;
        seconds_sum += minutes * 60;
        seconds_sum += seconds;

        let seconds_sum = (seconds_sum as f64) + (1.0_f64 / milliseconds_parts as f64);

        Ok(FFMpegDuration(Duration::from_secs_f64(seconds_sum)))
    }
}

pub async fn execute_job_using_ffmpeg(
    job: job::TranscoderJob,
    on_progress: Box<dyn Fn(ProgressUpdate) -> ()>,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("ffmpeg");

    // allow overwriting
    command.arg("-y");

    // no stdin, as ffmpeg is run in the background with no user interaction possible
    command.arg("-nostdin");

    // custom progress handling, piping to stdout
    command.args(["-progress", "-", "-nostats"]);

    command.args(["-i", job.input_uri()]);
    let new_dimensions = job.new_dimensions();

    command.args([
        "-s",
        &format!("{}x{}", new_dimensions.width(), new_dimensions.height()),
    ]);

    if let Some(audio_codec) = job.audio_codec() {
        command.args(["-c:a", audio_codec]);
    }

    command.arg(job.output_uri());

    // setup stderr and stdout prior to spawning

    command.stderr(Stdio::piped());
    command.stdout(Stdio::piped());

    let mut child = command.spawn().expect("failed to spawn command");

    let stderr = child
        .stderr
        .take()
        .expect("child did not have a handle to stderr");

    let mut stderr_reader = BufReader::new(stderr).lines();

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();

    let file_duration: Arc<RwLock<Option<FFMpegDuration>>> = Arc::new(RwLock::new(None));

    let file_duration_clone = file_duration.clone();

    // spawn a task that's concerned with gathering the duration by observing stderr output
    tokio::spawn(async move {
        while let Some(line) = stderr_reader.next_line().await.unwrap() {
            if let Some(captures) = DURATION_REGEX.captures(&line) {
                let duration = captures[1]
                    .parse::<FFMpegDuration>()
                    .expect("duration is valid");

                let mut writer = file_duration.write().await;
                *writer = Some(duration);

                // we have done our job here, let's terminate
                break;
            }
        }
    });

    let mut progress_information: HashMap<String, String> = HashMap::new();

    while let Some(line) = stdout_reader.next_line().await.unwrap() {
        // the `progress={continue, end}` is always the last. Everything before that belongs together
        // so as soon as we reached this line, we can process what we have received and notify about the progress
        if line.starts_with("progress=") {
            dbg!(&progress_information);
            if let Some(out_time_ms) = progress_information
                .get("out_time")
                .and_then(|x| x.parse::<FFMpegDuration>().ok())
            {
                if let Some(duration) = *file_duration_clone.read().await {
                    // yay we have all the information we need, so we can calculate the current progress
                    // and fire the event
                    on_progress(ProgressUpdate {
                        percentage: out_time_ms.0.as_secs_f64() / duration.0.as_secs_f64(),
                        state: ProgressState::Started,
                    });
                } else {
                    // we cannot say how far we have got, yet we can send a pulse we're still alive
                    on_progress(ProgressUpdate {
                        percentage: f64::NAN,
                        state: ProgressState::Started,
                    });
                }
            }

            progress_information.clear();
        } else {
            let (key, value) = line.split_once('=').expect("ffmpeg upholds its invariants");
            progress_information.insert(key.into(), value.into());
        }
    }

    let status = child
        .wait()
        .await
        .expect("child process encountered an error");

    // TODO: Handle other statusses gracefully
    assert_eq!(status.code(), Some(0));

    // assume we have finished successfully, send a last update
    on_progress(ProgressUpdate {
        percentage: 1.,
        state: ProgressState::Finished,
    });

    Ok(())
}
