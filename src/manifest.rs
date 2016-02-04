extern crate elf;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use std::str;

/// A Manifest includes paths
/// to binaries and their dependencies
#[derive(Default)]
pub struct Manifest {
    pub paths: HashSet<String>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest { ..Default::default() }
    }

    pub fn dirs(&self) -> HashSet<&Path> {
        let mut ds = HashSet::new();
        for p in self.paths.iter().filter_map(|p| Path::new(p).parent()) {
            ds.insert(p);
        }
        ds
    }

    pub fn include(&mut self, bin: &str) {
        // linux-vdso.so.1 (0x00007ffe56dcf000)
        let abs = Regex::new(r"\s+(?P<bin>\S+)\s+\(\S+\)").unwrap();
        // libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fdd79aa5000)
        let link = Regex::new(r"\s+\S+\s+=>\s+\((?P<bin>\S+)\)\s+\(\S+\)").unwrap();
        self.paths.insert(bin.to_owned());
        let path: PathBuf = From::from(bin.clone());
        if let Some(file) = elf::File::open_path(&path).ok() {
            if let Some(s) = file.get_section(".interp") {
                let interp = str::from_utf8(s.data.as_ref()).unwrap().trim_right_matches('\0');
                self.paths.insert(interp.to_owned());
                let output = Command::new(interp).arg("--list").arg(bin.clone()).output().unwrap();
                let stroutput = String::from_utf8_lossy(&output.stdout);
                let deps = stroutput.lines()
                                    .into_iter()
                                    .filter_map(|l| {
                                        abs.captures(l)
                                           .or(link.captures(l))
                                           .map(|caps| caps.name("bin"))
                                    })
                                    .filter_map(|s| s)
                                    .collect::<Vec<&str>>();
                for d in deps {
                    // http://man7.org/linux/man-pages/man7/vdso.7.html
                    if !d.contains("vdso") {
                        self.paths.insert(d.to_owned());
                    }
                }
            }
        }
    }
}
