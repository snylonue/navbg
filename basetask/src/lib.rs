use ngtools;
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Basetask {
	name: String,
	priority: i32,
	progress: ngtools::Progress,
	create_time: DateTime<Utc>,
	tid: u64
}

impl Basetask {
	pub fn new(name: String, priority: i32, progress: ngtools::Progress) -> Basetask {
		Basetask {name, priority, progress, create_time: Utc::now(), tid: ngtools::random_hash()}
	}
	pub fn from_details(name: String, priority: i32, progress: ngtools::Progress, create_time: DateTime<Utc>, tid: u64) -> Basetask {
		Basetask {name, priority, progress, create_time, tid}
	}
}