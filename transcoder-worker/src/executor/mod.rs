mod ffmpeg;

use common::job;
use common::progress::ProgressUpdate;
use std::error::Error;
use tracing::info;

pub async fn execute_job(job: job::TranscoderJob) -> Result<(), Box<dyn Error>> {
    // TODO: Add other ways to execute the job
    info!("picked up new job, scaling to {:?}", job.new_dimensions());

    ffmpeg::execute_job_using_ffmpeg(
        job,
        Box::new(|progress: ProgressUpdate| {
            info!("current progress: {:.2} %", progress.percentage * 100.);
        }),
    )
    .await
}
