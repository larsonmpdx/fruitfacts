// walk through all .json5 references to find any marked "thumbnail: website"
// call out to /backend/web_screenshot/index.js to generate puppeteer screenshots for their reference urls
use std::fs;
use std::path::Path;
extern crate clap;
use anyhow::{anyhow, Result};
use clap::{crate_version, Arg, Command as ClapApp};
use std::io::Write;

#[cfg(feature = "binaries")]
fn web_address_to_png(web_address: &str, screenshot_path: &Path) -> Result<()> {
    let output = std::process::Command::new("node ../../web_screenshot/index.js")
        .arg(web_address)
        .arg(screenshot_path)
        .output()
        .expect("failed to run node screenshot process");

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    if output.status.code() != Some(0) {
        return Err(anyhow!("screenshot process failed"));
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct CollectionJson {
    thumbnail: Option<String>,
    url: Option<String>,
}

#[cfg(feature = "binaries")]
fn main() {
    // find all *.json5 recursively in plant_database/ and see which ones are marked "thumbnail: website"
    // for any without an existing thumbnail, create one with the same path but changed extension

    let matches = ClapApp::new("")
        .version(crate_version!())
        .arg(
            Arg::new("redo_all")
                .short('r')
                .long("redo_all")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("redo all thumbnails"),
        )
        .get_matches();

    let database_dir = harvest_chart_server::import_db::get_database_dir().unwrap();

    for entry in walkdir::WalkDir::new(database_dir.join("references"))
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let input_path = entry.path();

        if fs::metadata(input_path).unwrap().is_file() // filenames can't be >260 chars here without help - probably fixed in rust 1.58 - https://github.com/rust-lang/rust/issues/67403
        && input_path.extension().unwrap().to_str().unwrap() == "json5"
        {
            let contents = fs::read_to_string(input_path).unwrap();

            let collection: CollectionJson = json5::from_str(&contents).unwrap_or_else(|error| {
                panic!(
                    "couldn't parse json in file {} {}",
                    input_path.display(),
                    error
                );
            });

            if collection.thumbnail != Some("website".to_string()) {
                continue; // if not marked, skip, maybe it has a pdf screenshot instead
            }

            // todo - fetch web address from the "url" field (and use only the first part before spaces)

            let web_address: String = collection
                .url
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .to_string();

            println!(
                "reference: {}\n\turl: {}",
                input_path.display(),
                web_address
            );

            let mut screenshot_path = input_path.to_path_buf();
            screenshot_path.set_extension("png");

            if screenshot_path.exists() && !matches.get_flag("redo_all") {
                println!("png already exists");
            } else if let Err(error) = web_address_to_png(&web_address, &screenshot_path) {
                println!("error: {error:?}");
            }
        }
    }
}

#[cfg(not(feature = "binaries"))]
fn main() {
    // empty for when the feature isn't selected
    println!("feature \"binaries\" not selected, this tool was not built");
}
