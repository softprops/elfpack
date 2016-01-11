extern crate elf;
extern crate regex;
extern crate users;

use std::collections::HashSet;

pub mod manifest;
pub use manifest::Manifest;

#[derive(Default)]
pub struct Packer {
    users: HashSet<String>,
    groups: HashSet<String>,
    paths: HashSet<(String, String)>,
    manifest: HashSet<String>
}

impl Packer {
    pub fn new() -> Packer {
        Packer {
            ..Default::default()
        }
    }

    /// add a new user
    pub fn adduser(&mut self, user: &str) {
        if user.contains(":") {
            self.users.insert(user.to_owned());
        } else {
            if let Some(u) = users::get_user_by_name(user) {
                // todo: write full row
            }
        }
    }

    /// add a new group
    pub fn addgroup(&mut self, group: &str) {
        if group.contains(":") {
            self.groups.insert(group.to_owned());
        } else {
            if let Some(grp) = users::get_group_by_name(group) {
                // todo: write full row
            }
        }
    }

    /// add a new file
    pub fn addfile(&mut self, file: &str, dest: Option<&str>) {
        let destination = dest.unwrap_or(file);
        self.paths.insert((file.to_owned(), destination.to_owned()));
    }
}


#[test]
fn it_works() {}
