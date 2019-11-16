use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;

use ngtools;
use basetask;

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
    eps: HashMap<String, HashMap<String, Ep>>,
    //types: Vec<String>,
}

impl Ep {
    pub fn new<S>(number: S, name: S, status: Status, ep_type: S) -> Ep
    where S: Into<String>
    {
        Ep { number:number.into(), name:name.into(), status, ep_type:ep_type.into() }
    }
    pub fn number(&self) -> &String {
        &self.number
    }
    pub fn set_number<S>(&mut self, new_num: S) 
    where S: Into<String>
    {
        self.number = new_num.into();
    }
}
impl Default for Ep {
    fn default() -> Ep {
        Ep::new("1", "", Status::Watching, "ep")
    }
}
impl Eps {
    pub fn new() -> Eps {
        Eps { eps: HashMap::new() }
    }
    pub fn from_vec(veceps: Vec<Ep>) -> Eps {
        let mut eps = HashMap::new();
        //{ type: Hashmap }
        for i in veceps.iter() {
            eps.insert(i.ep_type.clone(), HashMap::new());
            //types.push(i.ep_type);
        }
        //{ type: Hashmap: {number: Ep} }
        for i in veceps {
            let epty = eps.get_mut(&i.ep_type).unwrap();
            epty.insert(i.number().clone(), i);
        }
        Eps { eps }
    }
}
impl basetask::Modify for Eps {
    type Task = Ep;
    type Key = String;

    fn insert(&mut self, new_task: Ep) -> Option<Ep> {
        let new_hm: HashMap<String, Ep> = HashMap::new();
        self.eps.entry(new_task.ep_type.clone()).or_insert(new_hm);
        let epty = self.eps.get_mut(&new_task.ep_type).unwrap();
        epty.insert(new_task.number().clone(), new_task)
    }
    fn pop(&mut self, key: &String) -> Option<Ep> {
        //unfinished
        Some(Ep::default())
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