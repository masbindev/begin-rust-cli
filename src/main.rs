#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Simple CLI Program made with Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// pattern to search for
    // #[arg(short='p', long="pattern")]
    pattern: String,

    /// file path file to read
    // #[arg(short='f', long="filepath")]
    path: std::path::PathBuf
}

fn main() {
    // println!("Hello, world!");
    let args = Cli::parse();
    println!("Pattern : {}, Path : {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path);

    match content {
        Ok(content) => {
            let mut count_line = 0;
            for line in content.lines(){
                count_line += 1;
                if line.contains(&args.pattern){
                    println!("{}|{}", count_line, line);
                }
            }
        },
        Err(error) => {println!("Something wrong when processing {:?}. {}", &args.path, error); }
    }

    // refactor using BufReader
    // let f = File::open(&args.path)?;
    // let mut reader = BufReader::new(f);
    
    // let mut count_line = 0;
    // for line in reader.lines(){
    //     count_line += 1;
    //     // println!("{:?}", line);

    //     if line.as_ref().unwrap().contains(&args.pattern){
    //         println!("{}|{}", count_line, line.unwrap());
    //     }
    // }

    // Ok(())

}
