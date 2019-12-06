use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map;
use std::slice;

use ngtools;
use basetask;
use basetask::Modify;
use basetask::Read;

static TYPED_INIT: Vec<Episode> = Vec::new();

macro_rules! into {
    ($($s: ident),*) => {
        $(let $s = $s.into();)*
    };
}
macro_rules! optn {
    ($x: expr) => {
            match $x {
                Some(some) => some,
                None => return None,
            };
    };
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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
pub struct Iter<'a> {
    types: hash_map::Values<'a, String, Vec<Episode>>,
    typed: slice::Iter<'a, Episode>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episodes {
    eps: HashMap<String, Vec<Episode>>,
}

impl Epinfo {
    pub fn with_ep<S>(chap: S) -> Epinfo
        where S: Into<String>
    {
        Epinfo { ep_type: String::from("season 1"), chap: chap.into() }
    }
}
impl Episode {
    pub fn new<S>(chap:S, ep_type: S, name: S, status: Status) -> Episode
        where S: Into<String>
    {
        into!(chap, name, ep_type);
        Episode {chap, ep_type, name, status}
    }
}
impl ngtools::Json for Episode {}
impl Default for Episode {
    fn default() -> Episode {
        Episode::new("1", "season 1", "", Status::Unwatched)
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
        for i in self.eps.values() {
            length += i.len();
        }
        length
    }
    pub fn iter(&self) -> Iter {
        Iter { types: self.eps.values(), typed: TYPED_INIT.iter() }
    }
}
impl ngtools::Json for Episodes {}
impl Modify for Episodes {
    type Task = Episode;
    type Key = Epinfo;

    fn insert(&mut self, new_task: Self::Task) -> Option<Self::Task> {
        let etp = self.eps.entry(new_task.ep_type.clone()).or_insert(vec![]);
        match etp.iter().position(|ep| { ep.chap == new_task.chap }) {
            Some(indx) => {
                //[.., old_task, .., new_task] ->[.., new_task, ..]
                etp.push(new_task);
                Some(etp.swap_remove(indx))
            },
            None => {
                etp.push(new_task);
                None
            },
        }
    }
    fn pop(&mut self, key: &Self::Key) -> Option<Self::Task> {
        let etp = optn!(self.eps.get_mut(&key.ep_type));
        let indx = optn!(etp.iter().position(|ep| { ep.chap == key.chap }));
        Some(etp.remove(indx))
    }
}
impl Read for Episodes {
    type Task = Episode;
    type Key = Epinfo;

    fn get(&self, key: &Self::Key) -> Option<&Self::Task> {
        let etp = optn!(self.eps.get(&key.ep_type));
        etp.iter().find(|ep| { ep.chap == key.chap })
    }
}
impl<'a> IntoIterator for &'a Episodes {
    type Item = &'a Episode;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Episode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.typed.next() {
            Some(v) => return Some(&v),
            None => {
                self.typed = optn!(self.types.next()).iter();
                self.typed.next()
            }
        }
    }
}