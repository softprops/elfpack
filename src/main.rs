extern crate elf;
extern crate regex;

use std::collections::HashSet;
use std::path::PathBuf;
use std::env;
use std::process::Command;
use regex::Regex;

#[derive(Default)]
struct Manifest {
    pack: HashSet<String>
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            ..Default::default()
        }
    }
    pub fn include(&mut self, bin: &str) {
        // linux-vdso.so.1 (0x00007ffe56dcf000)
        let abs = Regex::new(r"\s+(?P<bin>\S+)\s+\(\S+\)").unwrap();
        // libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fdd79aa5000)
        let link = Regex::new(r"\s+\S+\s+=>\s+\((?P<bin>\S+)\)\s+\(\S+\)").unwrap();
        self.pack.insert(bin.to_owned());
        let path: PathBuf = From::from(bin.clone());
        if let Some(file) = elf::File::open_path(&path).ok() {
            if let Some(s) = file.get_section(".interp") {
                let interp = std::str::from_utf8(s.data.as_ref()).unwrap().trim_right_matches('\0');
                self.pack.insert(interp.to_owned());
                let output = Command::new(interp).arg("--list").arg(bin.clone()).output().unwrap();
                let stroutput = String::from_utf8_lossy(&output.stdout);
                let deps = stroutput.lines().into_iter().filter_map(|l| {
                    abs.captures(l).or(link.captures(l)).map(|caps| caps.name("bin"))
                }).filter_map(|s|s).collect::<Vec<&str>>();
                for d in deps {
                    self.pack.insert(d.to_owned());
                }
            }
        }
    }
}

fn main() {
    if let Some(bin) = env::args().nth(1) {
        let mut manifest = Manifest::new();
        manifest.include(bin.as_ref());
        for b in manifest.pack {
            println!("{}", b)
        }
    }
}
