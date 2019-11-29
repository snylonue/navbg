use chrono;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map;

use ngtools;
use basetask;
use basetask::Tid;
use basetask::Modify;
use basetask::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    Wish,
    Watching,
    Watched,
    Hold,
    Dropped,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episode {
    number: u64,
    pub chap: String,
    pub name: String,
    pub status: Status,
    pub ep_type: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episodes {
    eps: HashMap<u64, Episode>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub eps: Episodes,
    pub status: Status,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    tid: u64,
}

impl Episode {
    pub fn new<S>(chap:S, name: S, status: Status, ep_type: S) -> Episode
        where S: Into<String>
    {
        Episode { number: ngtools::random_hash(), chap: chap.into(), name: name.into(), status, ep_type: ep_type.into() }
    }
    pub fn number(&self) -> u64 {
        self.number
    }
    pub fn set_number(&mut self, new_num: u64) 
    {
        self.change_tid_v(new_num);
    }
}
impl Default for Episode {
    fn default() -> Episode {
        Episode::new("1", "", Status::Watching, "ep")
    }
}
impl Tid for Episode {
    fn tid(&self) -> u64 {
        self.number
    }
    fn change_tid(&mut self) {
        self.number = ngtools::random_hash();
    }
    fn change_tid_v(&mut self, tid: u64) {
        self.number = tid;
    }
}
impl Episodes {
    pub fn new() -> Episodes {
        Episodes { eps: HashMap::new() }
    }
    pub fn from_vec(veceps: Vec<Episode>) -> Episodes {
        let mut eps = Episodes::new();
        for i in veceps {
            eps.insert(i);
        }
        eps
    }
    pub fn types(&self) -> hash_map::Keys<u64, Episode> {
        self.eps.keys()
    }
    pub fn len(&self) -> usize {
        self.eps.len()
    }
}
impl Modify for Episodes {
    type Task = Episode;
    type Key = u64;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task> {
        self.eps.insert(new_task.number, new_task)
    }
    fn pop(&mut self, key: &Self::Key) -> Option<Self::Task> {
        self.eps.remove(key)
    }
}
impl Read for Episodes {
    type Task = Episode;
    type Key = u64;

    fn get(&self, key: &Self::Key) -> Option<&Self::Task> {
        self.eps.get(key)
    }
}
impl Video {
    //field progress is autoly built
    pub fn new<S>(name: S, eps: Episodes) -> Video
        where S: Into<String>
    {
        let progress = ngtools::Progress::new(0, eps.len() as u32);
        Video { name: name.into(), eps, status: Status::Watching, progress, create_time: Utc::now(), tid: ngtools::random_hash() }
    }
    pub fn from_details<S>(name: S, eps: Episodes, status: Status, create_time: chrono::DateTime<Utc>, tid: u64) -> Video
        where S: Into<String>
    {
        let progress = ngtools::Progress::new(0, eps.len() as u32);
        Video { name: name.into(), eps, status, progress, create_time, tid }
    }
}
impl Tid for Video {
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