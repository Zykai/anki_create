use core::panic;
use std::{fs, collections::HashMap, pin};
#[derive(Clone)]
pub struct DictonaryEntry {
    pub trad: String,
    pub simp: String,
    pub pinyin: String,
    pub translation: String,
}

impl DictonaryEntry {
    fn new(trad: String, simp: String, pinyin: String, translation: String) -> DictonaryEntry {
        DictonaryEntry {trad, simp, pinyin, translation}
    }
}

fn convert_pinyin(pinyin: &str) -> String {
    let pinyin = pinyin.replace("u:", "ü").replace(" r5", "");
    let parts = pinyin.split(" ");
    let replace_list = [
        ["ā", "á", "ǎ", "à"],
        ["ü", "ǘ", "ǚ", "ǜ"],
        ["ē", "é", "ě", "è"],
        ["ī", "í", "ǐ", "ì"],
        ["ū", "ú", "ǔ", "ù"],
        ["ō", "ó", "ǒ", "ò"],
    ];
    let replace = ["a", "ü", "e", "i", "u", "o"];
    let mut res: Vec<_> = vec![];
    for p in parts {
        let number = p.chars().last().unwrap().to_digit(10);
        let mut has_number = true;
        let number = {
            if let Some(n) = number {
                n
            } else {
                has_number = false;
                100
            }
        };
        let transform = {
            if has_number {
                let index: usize = {
                    if p.contains("a") {
                        0
                    } else if p.contains("ü") {
                        1
                    } else if p.contains("e") {
                        2
                    } else if p.contains("i") {
                        3
                    } else if p.contains("u") {
                        4
                    } else if p.contains("o") {
                        5
                    } else {
                        0
                    }
                };
                let withoutnumber: String= p.chars().take(p.len()-1).collect();
                if number == 5 {
                    withoutnumber
                } else {
                    let dif = has_number as u32;
                    withoutnumber.replace(replace[index], replace_list[index][(number-dif) as usize])
                }
            } else {
                p.to_owned()
            }
        };
        res.push(transform);
    }
    return res.join("'")
}

fn convert_translation(translation: &str) -> String {
    let translation = translation.trim_matches('/');
    let translation: Vec<_> = translation.split('/').collect();
    translation.join(" ◆ ")
}

pub fn read_dict() -> HashMap<String, DictonaryEntry> {
    let dict = fs::read_to_string("res/cedict_ts.u8").unwrap();
    let lines = dict.lines().to_owned();
    let mut dict = HashMap::new();
    for line in lines {
        let chars = line.chars();
        let mut index = 0;
        let v: Vec<_> = chars.collect();
        if v[0] == '#' {
            continue;
        }
        let mut trad = vec![];
        let mut simplified = vec![];
        let mut pinyin = vec![];
        let mut translation = vec![];
        while v[index] != ' ' {
            trad.push(v[index]);
            index += 1;
        }
        index += 1;
        while v[index] != ' ' {
            simplified.push(v[index]);
            index += 1;
        }
        index += 2;
        while v[index] != ']' {
            pinyin.push(v[index]);
            index += 1;
        }
        index += 2;
        while index < v.len() {
            translation.push(v[index]);
            index += 1;
        }
        let trad : String = trad.into_iter().collect();
        let simplified : String = simplified.into_iter().collect();
        let pinyin : String = pinyin.into_iter().collect();
        let pinyin = convert_pinyin(&pinyin);
        let pinyin = pinyin.replace(" ", "'");
        let translation : String = translation.into_iter().collect();
        let translation = convert_translation(&translation);
        let entry = DictonaryEntry::new(trad, simplified, pinyin, translation);
        dict.insert(entry.simp.clone(), entry);
    }
    dict
}