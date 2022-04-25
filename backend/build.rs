extern crate dotenv_build;
extern crate dotenv;
use dotenv::dotenv;
use std::process::Command;

// gather build-time data and put it into some env vars
fn main() {
    dotenv().ok(); // load env vars

    dotenv_build::output(dotenv_build::Config::default()).unwrap(); // loads the .env file automatically
    {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap();
        println!(
            "cargo:rustc-env=GIT_HASH={}",
            String::from_utf8(output.stdout).unwrap()
        );
    }
    {
        let output = Command::new("git")
            .args(&["show", "-s", "--format=%ct"])
            .output()
            .unwrap();
        println!(
            "cargo:rustc-env=GIT_UNIX_TIME={}",
            String::from_utf8(output.stdout).unwrap()
        );
    }
    {
        let output = Command::new("git")
            .args(&["rev-list", "--count", "main"])
            .output()
            .unwrap();
        println!(
            "cargo:rustc-env=GIT_MAIN_COMMIT_COUNT={}",
            String::from_utf8(output.stdout).unwrap()
        );
    }
}
