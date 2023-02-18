mod dimensions;
mod executor;
mod job;
mod progress;

use crate::dimensions::Dimensions;
use crate::job::TranscoderJob;

#[tokio::main]
async fn main() {
    let job = TranscoderJob::new(
        "./bla.mp4".into(),
        "./transcoded.mp4".into(),
        Dimensions::new(320, 240),
        Some("copy".into()),
    );

    executor::execute_job(job).await.unwrap();
}
