// look in /frontend/public/data/ for any thumbnails that don't match a reference .json5 in /plant_database/references/
// and delete them

#[cfg(feature = "binaries")]
fn main() {
    // todo
}

#[cfg(not(feature = "binaries"))]
fn main() {
    // empty for when the feature isn't selected
    println!("feature \"binaries\" not selected, this tool was not built");
}
