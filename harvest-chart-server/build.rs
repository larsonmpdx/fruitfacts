use std::process::Command;
fn main() {
    {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", String::from_utf8(output.stdout).unwrap());
    }
    {
        let output = Command::new("git")
            .args(&["show", "-s", "--format=%ct"])
            .output()
            .unwrap();
        println!("cargo:rustc-env=GIT_UNIX_TIME={}", String::from_utf8(output.stdout).unwrap());
    }
    {
        let output = Command::new("git")
            .args(&["rev-list", "--count", "main"])
            .output()
            .unwrap();
        println!("cargo:rustc-env=GIT_MAIN_COMMIT_COUNT={}", String::from_utf8(output.stdout).unwrap());
    }
}
