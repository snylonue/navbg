use ngtools;
use serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Basetask {
    pub name: String,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: DateTime<Utc>,
    pub tid: u64,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Basetasks {
    task: HashMap<u64, Basetask>,
}

pub trait ChangeTid {
    fn change_tid(&mut self);
    fn change_tid_v(&mut self, tid: u64);
}
pub trait Modify {
    type Task;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task>;
    fn pop(&mut self, tid: u64) -> Option<Self::Task>;
}
pub trait Read {
    fn get(&self);
    fn search(&self, tid: u64);
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
impl Basetasks {
    //from_array() seems slow because of clone()
    //need to optimize
    pub fn new() -> Basetasks {
        Basetasks {task: HashMap::new()}
    }
    pub fn from_vec(vectasks: Vec<Basetask>) -> Basetasks {
        let mut task = HashMap::new();
        for i in vectasks {
            task.insert(i.tid, i);
        }
        Basetasks {task}
    }
    //may be removed
    pub fn from_array(arrtasks: &[Basetask]) -> Basetasks {
        let mut task = HashMap::new();
        for i in arrtasks.iter() {
            task.insert(i.tid, i.clone());
        }
        Basetasks {task}
    }
    pub fn tasks(&self) -> hash_map::Keys<u64, Basetask> {
        self.task.keys()
    }
    pub fn tids(&self) -> hash_map::Values<u64, Basetask> {
        self.task.values()
    }
    pub fn len(&self) -> usize {
        self.task.len()
    }
}
impl Modify for Basetasks {
    type Task = Basetask;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task> {
        self.task.insert(new_task.tid, new_task)
    }
    fn pop(&mut self, tid: u64) -> Option<Self::Task> {
        self.task.remove(&tid)
    }
}