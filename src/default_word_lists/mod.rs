/// Default word lists available on the [EFF site](https://www.eff.org/dice)
///
mod long_list;
mod short_list;
mod short_list_with_longer_words;

use std::str::FromStr;

#[derive(Debug)]
pub enum WordList {
    Long,
    Short,
    ShortWithLongerWords,
}

impl FromStr for WordList {
    type Err = String;
    fn from_str(input: &str) -> Result<WordList, String> {
        Ok(match input {
            "long" => WordList::Long,
            "shortest" => WordList::Short,
            "short" => WordList::ShortWithLongerWords,
            _ => return Err("".into()),
        })
    }
}

impl WordList {
    pub fn get(&self) -> (usize, Vec<(u32, &'static str)>) {
        match *self {
            WordList::Long => (5, load_long_list()),
            WordList::Short => (4, load_shortest_list()),
            WordList::ShortWithLongerWords => (4, load_short_list()),
        }
    }
}

type Entry = (u32, &'static str);

fn load_long_list() -> Vec<Entry> {
    let mut vector = Vec::with_capacity(long_list::WORD_LIST.len());
    for i in 0..long_list::WORD_LIST.len() {
        vector.push(long_list::WORD_LIST[i]);
    }
    vector
}
fn load_short_list() -> Vec<Entry> {
    let mut vector = Vec::with_capacity(short_list_with_longer_words::WORD_LIST.len());
    for i in 0..short_list_with_longer_words::WORD_LIST.len() {
        vector.push(short_list_with_longer_words::WORD_LIST[i]);
    }
    vector
}
fn load_shortest_list() -> Vec<Entry> {
    let mut vector = Vec::with_capacity(short_list::WORD_LIST.len());
    for i in 0..short_list::WORD_LIST.len() {
        vector.push(short_list::WORD_LIST[i]);
    }
    vector
}
