use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use rand::Rng;
use serde::Serialize;
use serde::Deserialize;

const LETTERS: [char; 62] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Progress {
    finished: u32,
    total: u32,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct TimeLen {
    hour: i32,
    minute: i32,
    second: i32,
}

impl Progress {
    pub fn new(finished: u32,total: u32) -> Progress {
        Progress { finished, total}
    }
    pub fn finish(&mut self) {
        self.finished = self.total;
    }
    pub fn set_progress(&mut self, new_progress: u32) -> Result<(), &'static str> {
        if new_progress > self.total {
            Err("new progress is larger than total")
        } else {
            self.finished = new_progress;
            Ok(())
        }
    }
    pub fn set_total(&mut self, new_total: u32) -> Result<(), &'static str> {
        if new_total < self.finished {
            Err("new total is smaller than finished")
        } else {
            self.total = new_total;
            Ok(())
        }
    }
    pub fn finished(&self) -> u32 {
        self.finished
    }
    pub fn total(&self) -> u32 {
        self.total
    }
    pub fn status(&self) -> (u32, u32) {
        (self.finished, self.total)
    }
}
impl Default for Progress {
    fn default() -> Progress {
        Progress { finished: 0, total: 1 }
    }
}
impl TimeLen {
    //will be removed
    pub fn create() -> TimeLen {
        TimeLen { hour: 0, minute: 0, second: 0 }
    }
    pub fn from_tuple(tpl: (i32, i32, i32)) -> TimeLen {
        TimeLen::new(tpl.0, tpl.1, tpl.2)
    }
    pub fn new(hour: i32, minute: i32, second: i32) -> TimeLen {
        let mut timl = TimeLen { hour, minute, second };
        timl.simple();
        timl
    }
    pub fn total_seconds(&self) -> i32 {
        self.hour * 3600 + self.minute * 60 + self.second
    }
    pub fn hour(&self) -> i32 {
        self.hour
    }
    pub fn minute(&self) -> i32 {
        self.minute
    }
    pub fn second(&self) -> i32 {
        self.second
    }
    fn simple(&mut self) {
        if self.second.abs() >= 60 {
            self.minute += self.second / 60;
            self.second %= 60;
        }
        if self.minute.abs() >= 60 {
            self.hour += self.minute / 60;
            self.minute %= 60;
        }
    }
}
impl Default for TimeLen {
    fn default() -> TimeLen {
        TimeLen { hour: 0, minute: 0, second: 0 }
    }
}

pub fn random_hash() -> u64 {
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    let strlen = rng.gen_range(1, 64);
    for _i in 0..strlen {
        let pos = rng.gen_range(0, LETTERS.len());
        res.push(LETTERS[pos]);
    }
    let mut s = DefaultHasher::new();
    res.hash(&mut s);
    s.finish()
}