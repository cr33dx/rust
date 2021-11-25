
extern crate regex;
extern crate clap;

use regex::Regex;
use clap::Arg;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main(){
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    let lines_required = 2;
    let mut lines:Vec<String> = Vec::new();
    let args = clap::App::new("grep-lite")
        .version("0.0.1")
        .about("mini grep")
        .arg(Arg::with_name("pattern")
             .help("whats the pattern")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("file_name")
             .help("enter file name")
             .takes_value(true)
             .required(false))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let file_name = args.value_of("file_name").unwrap();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines(){
        lines.push(line.unwrap());
    }
//    loop {
//        let len = reader.read_line(&mut line).unwrap();
//        if len == 0{
// //            break
//        }
//        lines.push(line.to_string());
//        line.truncate(0);
//    }
    let joined_lines = lines.join("\n");
    
    let re = Regex::new(pattern).unwrap();
    
    for (i, line) in joined_lines.lines().enumerate(){
        match re.find(line){
            Some(_) => tags.push(i),
            None => (),
