use ngtools;
use basetask;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;

pub enum Status {
	Wish,
	Watching,
	Watched,
	Hold,
	Dropped,
}
pub struct Ep {
	number: String,
	pub name: String,
	pub status: Status,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    pub tid: u64,
}
pub struct Eps {
	eps: HashMap<String, Ep>,
}