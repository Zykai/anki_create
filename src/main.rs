use std::collections::HashSet;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use indicatif::ProgressBar;
use jieba_rs::Jieba;
use dict::DictonaryEntry;
mod dict;
use std::fs::File;
use dict::read_dict;

const FORBIDDEN :  [&'static str; 15] = [" ","，","\n","\r","。","“","”","一","‘","’","\r\n","、","？","=","：",];

fn analyze_book(strings: &Vec<&str>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for s in strings {
        *map.entry((*s).to_owned()).or_insert(0) += 1;
    }
    println!("Found {} different words", map.len());
    let mut count_vec: Vec<_> = map.iter().filter(|a| !FORBIDDEN.contains(&a.0.as_str())).collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for entry in &count_vec[0..100] {
        println!("{:?}", entry);
    }
    count_vec[0..100].iter().for_each(|x| println!("{:?}", x));
    return map;
}

fn jieba_test(sentence: &str) -> Vec<&str> {
    let mut jieba = Jieba::new();
    //jieba.add_word("魔药", None, None);
    //jieba.add_word("非凡者", None, None);
    //jieba.cut_all(sentence)
    jieba.cut(sentence, true)
    //let words = jieba.cut(sentence, false);
    //words
}

fn test_jieba() {
    let name = {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            args[1].to_owned()
        } else {
            panic!("Please specify file name (without .txt extension)")
        }
    };
    let mut s = fs::read_to_string(format!("res/{name}.txt")).unwrap();
    s.retain(|c| !c.is_whitespace());
    s = s.replace("。", "。\n");
    s = s.replace("？", "？\n");
    s = s.replace("！", "！\n");
    //let mut test = File::create(format!("res/debug.txt")).unwrap();;
    //test.write(s.as_bytes());
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    println!("Book length {len}");
    let dict = read_dict();
    let word_list = get_know();
    let mut map: HashMap<String, (String, u32, DictonaryEntry)> = HashMap::new();
    let mut no_ent: HashMap<String, u32> = HashMap::new();
    let lines: Vec<&str> = s.lines().collect();
    let len = lines.len();
    let bar = ProgressBar::new(len as u64);

    let res: Vec<&str> = jieba_test(&s);
    let jieba_lines: Vec<_> = res.split(|s| *s == "\n").collect();

    println!("{:?}", jieba_lines.len());
    //println!("{:?}", jieba_lines);
    let mut index = 0;
    for line in jieba_lines {
        for token in line {
            
            if FORBIDDEN.contains(&token) || word_list.contains(*token){
                continue;
            }
            if dict.contains_key(*token) {
                let r = map.entry(token.to_string()).or_insert((lines[index].to_string(),0, dict.get(*token).unwrap().clone()));
                r.1 += 1;
            } else {
                *no_ent.entry(token.to_string()).or_insert(0) += 1;
            }
        }
        index += 1;
        bar.inc(1);
    }
    bar.finish();
    let mut count_vec: Vec<_> = no_ent.iter().filter(|a| !FORBIDDEN.contains(&a.0.as_str())).collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut count_map: Vec<_> = map.iter().collect();
    count_map.sort_by(|a, b| b.1.1.cmp(&a.1.1));
    
    //println!("{:?}", count_map.iter().map(|f| f.0).collect::<Vec<_>>());
    println!("Found {} new words contained in the dictionary, out of {} sentences", count_map.len(), len);
    //println!("{:?}", count_vec);
    println!("Found {} words not contained in the dictionary", count_vec.len());

    let mut res = File::create(format!("res/{name}_output.txt")).unwrap();
    for entry in count_map {
        let string = &entry.1.0;
        let dict = &entry.1.2;
        let count = &entry.1.1;
        res.write(format!("{}\t{}\t{}\t{}\t{}\n", dict.simp, dict.trad, dict.pinyin, dict.translation, string).as_bytes()).unwrap();
    }
    //println!("{:?}", count_vec);

}

fn get_know() -> HashSet<String> {
    let f = fs::read_to_string("res/known.txt");
    if f.is_err() {
        return HashSet::new();
    }
    let f = f.unwrap();
    let x = | s: &str | {
        let split = s.split("\t");
        let v: Vec<&str> = split.collect();
        v[0].to_owned()
    };
    f.split("\n").map(x).collect()
}

fn main() {
    test_jieba()
    //analyze_guimi();
}