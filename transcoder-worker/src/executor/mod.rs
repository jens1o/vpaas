mod ffmpeg;

use crate::{job, progress::ProgressUpdate};
use std::{error::Error, io::Stdout, process::Stdio};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn execute_job(job: job::TranscoderJob) -> Result<(), Box<dyn Error>> {
    // TODO: Add other ways to execute the job
    ffmpeg::execute_job_using_ffmpeg(
        job,
        Box::new(|progress: ProgressUpdate| {
            println!("current progress: {:.2} %", progress.percentage * 100.);
        }),
    )
    .await
}
