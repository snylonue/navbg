use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map;

use ngtools;
use basetask;
use basetask::Tid;
use basetask::Modify;
use basetask::Read;

macro_rules! into {
    ($($s: ident),*) => {
        $(let $s = $s.into();)*
    };
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    Wish,
    Unwatched,
    Watching,
    Watched,
    Hold,
    Dropped,
}
#[derive(Debug, Clone)]
pub struct Epinfo {
    pub ep_type: String,
    pub chap: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub chap: String,
    pub ep_type: String,
    pub name: String,
    pub status: Status,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episodes {
    eps: HashMap<String, Vec<Episode>>,
}

impl Episode {
    pub fn new<S>(chap:S, name: S, status: Status, ep_type: S) -> Episode
        where S: Into<String>
    {
        into!(chap, name, ep_type);
        Episode {chap, name, status, ep_type}
    }
}
impl ngtools::Json for Episode {}
impl Default for Episode {
    fn default() -> Episode {
        Episode::new("1", "ep", "name", Status::Unwatched)
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
    pub fn pre_init(length: u32) -> Episodes {
        let mut eps = Episodes::new();
        for i in 1 ..= length {
            eps.insert(Episode { chap: i.to_string(), ..Default::default() });
        }
        eps
    }
    pub fn types(&self) -> hash_map::Keys<String, Vec<Episode>> {
        self.eps.keys()
    }
    pub fn len(&self) -> usize {
        let mut length = 0;
        for i in self.values() {
            length += i.len();
        }
        length
    }
}
impl ngtools::Json for Episodes {}
impl Modify for Episodes {
    type Task = Episode;
    type Key = Epinfo;

    //return None because of the require of trait Modify
    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task> {
        let etp = self.eps.entry(new_task.ep_type.clone()).or_insert(vec![]);
        etp.push(new_task);
        None
    }
    fn pop(&mut self, key: &Self::Key) -> Option<Self::Task> {
        let mut etp = match self.eps.get_mut(&key.ep_type) {
            Some(v) => v,
            None => return None,
        };
        let indx = match etp.iter().position(|ep| { ep.chap == key.chap }) {
            Some(n) => n,
            None => return None,
        };
        Some(etp.remove(indx))
    }
}
impl Read for Episodes {
    type Task = Episode;
    type Key = Epinfo;

    fn get(&self, key: &Self::Key) -> Option<&Self::Task> {
        let etp = self.eps.get(&key.ep_type).unwrap();
        etp.iter().find(|ep| { ep.chap == key.chap })
    }
}