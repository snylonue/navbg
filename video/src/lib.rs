pub mod episode;

use chrono;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;

use ngtools;
use ngtools::random_hash;
use basetask;
use basetask::Tid;
use episode::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub status: Status,
    pub create_time: chrono::DateTime<Utc>,
    eps: Episodes,
    progress: ngtools::Progress,
    tid: u64,
}

impl Video {
    ///Field progress is automatically built
    pub fn new<S>(name: S, eps: Episodes) -> Video
        where S: Into<String>
    {
        Video::with_details(name, eps, Status::Watching, Utc::now(), random_hash())
    }
    pub fn with_details<S>(name: S, eps: Episodes, status: Status, create_time: chrono::DateTime<Utc>, tid: u64) -> Video
        where S: Into<String>
    {
        let progress = ngtools::Progress::new(eps.watched(), eps.len() as u32);
        Video { name: name.into(), eps, status, progress, create_time, tid }
    }
    pub fn iter(&self) -> Iter {
        self.eps.iter()
    }
    pub fn len(&self) -> usize {
        self.progress.total() as usize
    }
    pub fn eps(&self) -> &Episodes {
        &self.eps
    }
    pub fn progress(&self) -> ngtools::Progress {
        self.progress
    }
    pub fn watched(&self) -> u32 {
        self.progress.finished()
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

    ///progress will be updated after modify
    fn insert(&mut self, new_item: Self::Item) -> Option<Self::Item> {
        let res = self.eps.insert(new_item);
        self.progress.set_progress(self.eps.watched(), self.eps.len() as u32);
        res
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Item> {
        let res = self.eps.remove(key);
        self.progress.set_progress(self.eps.watched(), self.eps.len() as u32);
        res
    }
}
impl<'a> IntoIterator for &'a Video {
    type Item = &'a Episode;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}