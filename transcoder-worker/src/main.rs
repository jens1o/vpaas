extern crate common;
extern crate redis;
extern crate tokio;

mod executor;

use redis::AsyncCommands;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    loop {
        let queue_and_job_data = con.blpop::<_, Vec<String>>("vpaas:queue", 0).await.unwrap();

        let job = serde_json::from_str(&queue_and_job_data[1])
            .expect("json job in queue must be deserializble to a TranscoderJob");

        executor::execute_job(job).await.unwrap();

        // TODO: Inform that we finished successfully
    }
}
