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
    tasks: HashMap<u64, T>,
}

pub trait Tid {
    fn tid(&self) -> u64;
    fn change_tid(&mut self) {
        self.change_tid_v(ngtools::random_hash());
    }
    fn change_tid_v(&mut self, tid: u64);
}
pub trait Modify {
    type Item;
    type Key;

    fn insert(&mut self, new_item: Self::Item) -> Option<Self::Item>;
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Item>;
}
pub trait Read {
    type Item;
    type Key;

    fn get(&self, key: &Self::Key) -> Option<&Self::Item>;
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
    pub fn new() -> Tasks<T> {
        Tasks { tasks: HashMap::new() }
    }
    pub fn from_vec(vectasks: Vec<T>) -> Tasks<T> {
        let mut tasks = Tasks::new();
        for i in vectasks {
            tasks.insert(i);
        }
        tasks
    }
    //may be removed
    pub fn from_array(arrtasks: &[T]) -> Tasks<T>
        where T: Clone
    {
        let mut tasks = Tasks::new();
        for i in arrtasks.iter() {
            tasks.insert(i.clone());
        }
        tasks
    }
    pub fn tasks(&self) -> hash_map::Keys<u64, T> {
        self.tasks.keys()
    }
    pub fn tids(&self) -> hash_map::Values<u64, T> {
        self.tasks.values()
    }
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn iter(&self) -> hash_map::Values<u64, T> {
        self.tasks.values()
    }
}
impl<T> ngtools::Json for Tasks<T> {}
impl<T> Modify for Tasks<T>
    where T: Tid
{
    type Item = T;
    type Key = u64;

    fn insert(&mut self, new_item: Self::Item) -> Option<Self::Item> {
        self.tasks.insert(new_item.tid(), new_item)
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Item> {
        self.tasks.remove(key)
    }
}
impl<T> Read for Tasks<T>
    where T: Tid
{
    type Item = T;
    type Key = u64;

    fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
        self.tasks.get(key)
    }
}
impl<'a, T> IntoIterator for &'a Tasks<T>
    where T: Tid
{
    type Item = &'a T;
    type IntoIter = hash_map::Values<'a, u64, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}