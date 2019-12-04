use serde::Serialize;
use serde::Deserialize;
use chrono;
use chrono::Utc;
use std::collections::HashMap;
use std::collections::hash_map;

use ngtools;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Basetask {
    pub name: String,
    pub priority: i32,
    pub progress: ngtools::Progress,
    pub create_time: chrono::DateTime<Utc>,
    tid: u64,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tasks<T> {
    task: HashMap<u64, T>,
}

pub trait Tid {
    fn tid(&self) -> u64;
    fn change_tid(&mut self) {
        self.change_tid_v(ngtools::random_hash());
    }
    fn change_tid_v(&mut self, tid: u64);
}
pub trait Modify {
    type Task;
    type Key;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task>;
    fn pop(&mut self, key: &Self::Key) -> Option<Self::Task>;
}
pub trait Read {
    type Task;
    type Key;

    fn get(&self, key: &Self::Key) -> Option<&Self::Task>;
}

impl Basetask {
    pub fn new<S>(name: S, priority: i32, progress: ngtools::Progress) -> Basetask
        where S: Into<String>
    {
        Basetask { name:name.into(), priority, progress, create_time: Utc::now(), tid: ngtools::random_hash() }
    }
    pub fn from_details<S>(name: S, priority: i32, progress: ngtools::Progress, create_time: chrono::DateTime<Utc>, tid: u64) -> Basetask
        where S: Into<String>
    {
        Basetask { name:name.into(), priority, progress, create_time, tid }
    }
}
impl Tid for Basetask {
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
impl ngtools::Json for Basetask {}
impl<T> Tasks<T> 
    where T: Tid
{
    //from_array() seems slow because of clone()
    //need to optimize
    pub fn new() -> Tasks<T> {
        Tasks { task: HashMap::new() }
    }
    pub fn from_vec(vectasks: Vec<T>) -> Tasks<T> {
        let mut task = Tasks::new();
        for i in vectasks {
            task.insert(i);
        }
        task
    }
    //may be removed
    pub fn from_array(arrtasks: &[T]) -> Tasks<T>
        where T: Clone
    {
        let mut task = Tasks::new();
        for i in arrtasks.iter() {
            task.insert(i.clone());
        }
        task
    }
    pub fn tasks(&self) -> hash_map::Keys<u64, T> {
        self.task.keys()
    }
    pub fn tids(&self) -> hash_map::Values<u64, T> {
        self.task.values()
    }
    pub fn len(&self) -> usize {
        self.task.len()
    }
}
impl<T> ngtools::Json for Tasks<T> {}
impl<T> Modify for Tasks<T>
    where T: Tid
{
    type Task = T;
    type Key = u64;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task> {
        self.task.insert(new_task.tid(), new_task)
    }
    fn pop(&mut self, key: &Self::Key) -> Option<Self::Task> {
        self.task.remove(key)
    }
}
impl<T> Read for Tasks<T>
    where T: Tid
{
    type Task = T;
    type Key = u64;

    fn get(&self, key: &Self::Key) -> Option<&Self::Task> {
        self.task.get(key)
    }
}
//impl<T> Iterator for Tasks<T> {}
