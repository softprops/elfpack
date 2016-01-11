extern crate elfpack;
extern crate regex;

use std::env;
use elfpack::Manifest;

fn main() {
    if let Some(bin) = env::args().nth(1) {
        let mut manifest = Manifest::new();
        manifest.include(bin.as_ref());
        println!("paths");
        for p in &manifest.paths {
            println!("{}", p);
        }
        println!("dirs");
        for d in &manifest.dirs() {
            println!("{:?} ", d);
        }
    }
}
