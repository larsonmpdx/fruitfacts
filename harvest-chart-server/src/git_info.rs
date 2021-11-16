// MIT or Apache from https://github.com/rustsec/rustsec/blob/main/rustsec/src/repository/git/modification_time.rs
// see discussion https://github.com/rust-lang/git2-rs/issues/588
// last updated Nov 2021 from version 740a1dc
// changed from indexing on a path to a string in order to handle git's lowercase/uppercase mixups
use git2::Error;

use git2::Time;
use std::{
    cmp::max,
    collections::HashMap,
    path::{Path},
};

// Tracks the time of latest modification of files in git
pub struct GitModificationTimes {
    mtimes: HashMap<String, Time>,
}

pub fn format_path(input: &str) -> String {
    str::replace(&input.to_lowercase(), r#"\"#, "/") // to_lowercase: git is slightly case-insensitive, and git logs will maintain the original case for a file. when we fix casing we want to still find the file
}

impl GitModificationTimes {
    // Performance: collects all modification times on creation
    // and caches them. This is more efficient for looking up lots of files,
    // but wasteful if you just need to look up a couple files
    pub fn new(path: &Path) -> Result<Self, Error> {
        // Sadly I had to hand-roll this; there is no good off-the-shelf impl
        // libgit2 has had a feature request for this for over a decade:
        // https://github.com/libgit2/libgit2/issues/495
        // as does git2-rs: https://github.com/rust-lang/git2-rs/issues/588
        // To make sure this works I've verified it against a naive shell script using `git log`
        // as well as `git whatchanged`
        let mut mtimes: HashMap<String, Time> = HashMap::new();
        let repo = git2::Repository::open(path)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;
            // Ignore merge commits (2+ parents) because that's what 'git whatchanged' does
            if commit.parent_count() <= 1 {
                let tree = commit.tree()?;
                let prev_tree = match commit.parent_count() {
                    1 => Some(commit.parent(0)?.tree()?), // Diff with the previous commit
                    0 => None, // We've found the initial commit, diff with empty repo
                    _ => unreachable!(), // Ruled out by the `if` above
                };
                let diff = repo.diff_tree_to_tree(prev_tree.as_ref(), Some(&tree), None)?;
                for delta in diff.deltas() {
                    let file_path = delta.new_file().path().unwrap();
                    let file_mod_time = commit.time();

                    mtimes
                        .entry(format_path(file_path.to_owned().to_str().unwrap()))
                        .and_modify(|t| *t = max(*t, file_mod_time))
                        .or_insert(file_mod_time);
                }
            }
        }
        Ok(GitModificationTimes { mtimes })
    }

    // Looks up the Git modification time for a given file path
    // The path must be relative to the root of the repository
    pub fn for_path(&self, path: &Path) -> Option<&Time> {
        let formatted = format_path(path.to_owned().to_str().unwrap());
        // println!(r#"looking up {}"#, formatted);
        self.mtimes.get(&formatted)
    }

    pub fn print(&self) {
        for (key, value) in &self.mtimes {
            println!("{:#?} {}", key, value.seconds());
        }
    }
}