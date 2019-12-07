pub mod episode;

use chrono;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;

use ngtools;
use basetask;
use basetask::Tid;
use episode::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub eps: Episodes,
    pub status: Status,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    tid: u64,
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
    pub fn iter(&self) -> Iter<> {
        self.eps.iter()
    }
}
impl ngtools::Json for Video {}
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
impl Default for Video {
    fn default() -> Self {
        Video::new("", Default::default())
    }
}
impl basetask::Modify for Video {
    type Item = Episode;
    type Key = Epinfo;

    fn insert(&mut self, new_task: Self::Item) -> Option<Self::Item> {
        self.eps.insert(new_task)
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Item> {
        self.eps.remove(key)
    }
}
impl<'a> IntoIterator for &'a Video {
    type Item = &'a Episode;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}