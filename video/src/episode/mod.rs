use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Episodes {
    eps: HashMap<String, Vec<Episode>>,
    types: Vec<String>,
}
pub struct Iter<'a> {
    items: &'a HashMap<String, Vec<Episode>>,
    types: slice::Iter<'a, String>,
    typed: slice::Iter<'a, Episode>,
}

impl Epinfo {
    pub fn new<S>(ep_type: S, chap: S) -> Epinfo
        where S: Into<String>
    {
        into!(ep_type, chap);
        Epinfo { ep_type, chap }
    }
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
    fn default() -> Self {
        Episode::new("1", "season 1", "", Status::Unwatched)
    }
}
impl Episodes {
    pub fn new() -> Episodes {
        Episodes { eps: HashMap::new(), types: Vec::new() }
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
    pub fn types(&self) -> &Vec<String> {
        &self.types
    }
    pub fn watched(&self) -> u32 {
        let mut counter = 0;
        for i in self.iter() {
            match i.status {
                Status::Watched => counter += 1,
                _ => (),
            };
        }
        counter
    }
    pub fn len(&self) -> usize {
        let mut length = 0;
        for i in self.eps.values() {
            length += i.len();
        }
        length
    }
    pub fn iter(&self) -> Iter {
        let types = self.types.iter();
        Iter { items: &self.eps, types, typed: TYPED_INIT.iter() }
    }
}
impl ngtools::Json for Episodes {}
impl Modify for Episodes {
    type Item = Episode;
    type Key = Epinfo;

    fn insert(&mut self, new_item: Self::Item) -> Option<Self::Item> {
        let etp = match self.eps.entry(new_item.ep_type.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                self.types.push(new_item.ep_type.clone());
                entry.insert(vec![])
            },
        };
        match etp.iter().position(|ep| { ep.chap == new_item.chap }) {
            Some(indx) => {
                //[.., old_item, ..], new_item -> [.., old_item, .., new_item] ->[.., new_item, ..], old_item
                etp.push(new_item);
                Some(etp.swap_remove(indx))
            },
            None => {
                etp.push(new_item);
                None
            },
        }
    }
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Item> {
        let etp = self.eps.get_mut(&key.ep_type)?;
        let indx = etp.iter().position(|ep| { ep.chap == key.chap })?;
        let remove = etp.remove(indx);
        if etp.len() == 0 {
            self.types.remove(self.types.iter().position(|ty| *ty == remove.ep_type).unwrap());
            self.eps.remove(&remove.ep_type);
        }
        Some(remove)
    }
}
impl Read for Episodes {
    type Item = Episode;
    type Key = Epinfo;

    fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
        self.eps.get(&key.ep_type)?.iter().find(|ep| { ep.chap == key.chap })
    }
}
impl Default for Episodes {
    fn default() -> Self {
        Episodes::new()
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
                let new_type = self.types.next()?;
                self.typed = self.items[new_type].iter();
                self.typed.next()
            }
        }
    }
}