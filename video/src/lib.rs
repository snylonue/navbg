use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map;

use ngtools;
use basetask;
use basetask::Modify;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    Wish,
    Watching,
    Watched,
    Hold,
    Dropped,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone)]
pub struct EpInfo {
    pub number: String,
    pub ep_type: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episode {
    chap: EpInfo,
    pub name: String,
    pub status: Status,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Video {
    pub name: String,
    pub eps: Episodes,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    pub tid: u64,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episodes {
    eps: HashMap<EpInfo, Episode>,
}

impl EpInfo {
    pub fn new<S>(number:S, ep_type:S) -> EpInfo
        where S: Into<String>
    {
        EpInfo { number: number.into(), ep_type: ep_type.into() }
    }
}
impl Episode {
    pub fn new<S>(chap: EpInfo, name: S, status: Status) -> Episode
        where S: Into<String>
    {
        Episode { chap, name:name.into(), status }
    }
    pub fn number(&self) -> &String {
        &self.chap.number
    }
    pub fn set_number<S>(&mut self, new_num: S) 
        where S: Into<String>
    {
        self.chap.number = new_num.into();
    }
    pub fn ep_type(&self) -> &String {
        &self.chap.ep_type
    }
    pub fn set_ep_type<S>(&mut self, new_ep_type: S) 
        where S: Into<String>
    {
        self.chap.ep_type = new_ep_type.into();
    }
    pub fn chap(&self) -> &EpInfo {
        &self.chap
    }

}
impl Default for EpInfo {
    fn default() -> EpInfo {
        EpInfo::new("1", "ep")
    }
}
impl Default for Episode {
    fn default() -> Episode {
        Episode::new(EpInfo::default(), "", Status::Watching)
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
    pub fn types(&self) -> hash_map::Keys<EpInfo, Episode> {
        self.eps.keys()
    }
}
impl Modify for Episodes {
    type Task = Episode;
    type Key = EpInfo;

    fn insert(&mut self, new_task: Episode) -> Option<Episode> {
        self.eps.insert(new_task.chap().clone(), new_task)
    }
    fn pop(&mut self, key: &EpInfo) -> Option<Episode> {
        self.eps.remove(key)
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