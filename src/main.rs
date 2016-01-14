extern crate clap;
extern crate elfpack;
extern crate regex;
extern crate users;

use std::{env, fs};
use elfpack::Manifest;
use clap::App;

fn main() {
    let matches = App::new("elfpack")
        .version(env!("CARGO_PKG_VERSION"))
        .about("packages elf binaries into tiny docker containers")
        .args_from_usage(
            "-u --user=[USER] 'sets to user to run as'
             -g --group=[GROUP] 'sets the group to run as'
             -c --command=[CMD] 'sets the docker default cmd to run'
             -e --entrypoint=[ENTRYPOINT] 'sets to docker entry point'
             -t --targetdir=[TARGET] 'sets the target dir to output Dockerfile to'
            <ELFBIN> 'the elf binary to pack'"
        )
        .get_matches();

    let bin = matches.value_of("ELFBIN").unwrap();
    let mut manifest = Manifest::new();
    manifest.include(bin.as_ref());
    for p in &manifest.paths {
        println!("copying {} -> ...", p);
        let _ = fs::copy(p, "target/docker/");
    }
}
