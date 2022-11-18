#![allow(non_snake_case)]

/// Recursive descent parser for grammer 
/// 
/// A  -> B C
/// B  -> B! | D
/// C  -> *A | epsilon
/// D  -> n  | (A)
/// 
/// After Resolving left recursive conflict 
/// 
/// A  -> B C
/// B  -> D B`
/// B` -> !B` | epsilon
/// C  -> *A | epsilon
/// D  -> n  | (A)

use lexer::Lexer;
use lexer::RecDes;
use std::collections::HashMap;
use std::{fs, io::Read};

fn ReadFileIntoStringVec(fileName: &String, sep: char) -> Vec<String> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .open(fileName)
        .ok()
        .unwrap();

    let mut content = String::new();

    _ = file.read_to_string(&mut content);
    content = String::from(content.trim());

    let filesub: Vec<_> = content.split(sep).collect();
    let mut res: Vec<String> = Vec::new();

    for value in filesub {
        res.push(String::from(value));
    }

    res
}

fn genSpace(count: usize) -> String {
    let mut res = String::from("");

    for _ in 0..count {
        res.push(' ');
    }

    res
}

fn main() {
    let fileName = String::from("./examples/input.txt");
    let fileContents = ReadFileIntoStringVec(&fileName, ';');

    let mut valueMap: HashMap<&String, bool> = HashMap::new();
    let mut lexer: Lexer;
    let mut maxcount = 11;

    for value in &fileContents {
        lexer = (value).into();
        valueMap.insert(value, RecDes::NStart(lexer));

        if value.len() > maxcount {
            maxcount = value.len();
        }
    }

    println!("ValuePassed{}Result\n",genSpace(maxcount-9));

    for mapSet in valueMap {
        let (value , bvalue) = mapSet;
        println!("{}{}{}",value,genSpace(maxcount-value.len()+2),bvalue);
    }

 
}
