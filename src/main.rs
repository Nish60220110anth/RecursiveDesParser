#![allow(non_snake_case)]

#[cfg(windows)]
const LINE_ENDING:&'static str ="\n";
#[cfg(not(windows))]
const LINE_ENDING:&'static str ="\r\n";

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
use std::env;
use std::path;

fn ReadFileIntoStringVec(filePath: &String, sep: char) -> Vec<String> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .open(filePath)
        .ok()
        .unwrap();

    let mut content = String::new();

    _ = file.read_to_string(&mut content);
    content = String::from(content.replace(LINE_ENDING, ""));

    let filesub: Vec<_> = content.split(sep).collect();
    let mut res: Vec<String> = Vec::new();

    for value in filesub {
        if value.eq("") {
            continue;
        }
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
    let argVec:Vec<String> = env::args().collect();

    if argVec.len() < 2 {
        panic!("path to input file not passed");
    }

    let inputPathString = &argVec[1];
    let inputPath = path::Path::new(inputPathString);

    if !inputPath.exists() {
        panic!("path to input file doesn't exists");
    }

    let filePath:String = String::from(inputPath.as_os_str().to_str().unwrap());
    let fileContents = ReadFileIntoStringVec(&filePath, ';');

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
