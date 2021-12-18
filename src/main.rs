#![allow(unused_imports)]
#![allow(dead_code)]

mod formatting;
mod misc;
use std::env;
use std::fs;
use crate::formatting::print_paths;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // The first argument is the path to the binary
    // and we don't need it.
    args.remove(0);

    let mut paths = fs::read_dir("src").unwrap();

    print_paths(&mut paths, &mut args);
    // println!("Copying the following files to {}", formatting::test());
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    // fs::create_dir()
}