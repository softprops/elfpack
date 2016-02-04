extern crate clap;
extern crate elfpack;
extern crate regex;
extern crate users;

use std::fs;
use std::path::Path;
use elfpack::Manifest;
use clap::App;

fn main() {
    let args = App::new("elfpack")
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


    fn cp<S,D>(path: S, target: D) where S: AsRef<Path>, D: AsRef<Path> {
        let src = path.as_ref().canonicalize().unwrap();
        let dest = target.as_ref().join(path.as_ref().to_string_lossy().trim_left_matches("/"));
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        println!("copying {:?} to {:?}", src, dest);
        fs::copy(src, dest).unwrap();
    }

    let bin = args.value_of("ELFBIN").unwrap();
    let target = Path::new(args.value_of("TARGET").unwrap_or("target/docker"));
    fs::create_dir_all(target).unwrap();

    let mut manifest = Manifest::new();
    manifest.include(bin.as_ref());

    for p in &manifest.paths {
        let path = Path::new(p);
        if path.is_relative() {
            let dest = target.join(path.file_name().unwrap());
            println!("copying manifest path {:?} to {:?}", path, dest);
            fs::copy(&path, dest).unwrap();
        } else {
            cp(path, target);
        }
    }
    for nss in vec!["libnss_dns.so.2", "libnss_files.so.2", "libnss_compat.so.2"] {
        for d in &manifest.dirs() {
            let try_path = d.join(nss);
            if try_path.exists() {
                cp(try_path, target);
            }
        }
    }
}
