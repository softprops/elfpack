extern crate elfpack;
extern crate regex;
extern crate users;

use std::env;
use elfpack::Manifest;

fn main() {
    use users::{get_user_by_uid, get_current_uid};
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hello, {}!", user.name);
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
