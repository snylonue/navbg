use ngtools;
use basetask;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    Wish,
    Watching,
    Watched,
    Hold,
    Dropped,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Ep {
    number: String,
    pub name: String,
    pub status: Status,
    pub ep_type: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub eps: Eps,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    pub tid: u64,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Eps {
    eps: HashMap<String, Ep>,
}

impl Ep {
    pub fn new(number: String, name: String, status: Status, ep_type: String) -> Ep {
        Ep {number, name, status, ep_type}
    }
    pub fn get_number(&self) -> &String {
    	&self.number
    }
    pub fn set_number(&mut self, new_num: String) {
    	self.number = new_num;
    }
}
impl Eps {
	pub fn new() -> Eps {
		Eps {eps: HashMap::new()}
	}
	pub fn from_vec(veceps: Vec<Ep>) -> Eps {
        let mut eps = HashMap::new();
        for i in veceps {
            eps.insert(i.get_number().clone(), i);
        }
        Eps {eps}
	}
}
impl basetask::Tid for Video {
    fn tid(&self) -> u64 {
        self.tid
    }
    fn change_tid(&mut self) {
        self.tid = ngtools::random_hash();
    }
    fn change_tid_v(&mut self, tid: u64) {
        self.tid = tid;
    }
}