use ngtools;
use serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Basetask {
    pub name: String,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: DateTime<Utc>,
    pub tid: u64
}

pub trait ChangeTid {
    fn change_tid(&mut self);
    fn change_tid_v(&mut self, tid: u64);
}

impl Basetask {
    pub fn new(name: String, priority: i32, progress: ngtools::Progress) -> Basetask {
        Basetask {name, priority, progress, create_time: Utc::now(), tid: ngtools::random_hash()}
    }
    pub fn from_details(name: String, priority: i32, progress: ngtools::Progress, create_time: DateTime<Utc>, tid: u64) -> Basetask {
        Basetask {name, priority, progress, create_time, tid}
    }
}
impl ChangeTid for Basetask {
    fn change_tid(&mut self) {
        self.tid = ngtools::random_hash();
    }
    fn change_tid_v(&mut self, tid: u64) {
        self.tid = tid;
    }
}