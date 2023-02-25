extern crate serde;
extern crate serde_json;

pub const QUEUE_NAME: &'static str = "vpaas:queue";

pub mod dimensions;
pub mod job;
pub mod progress;
