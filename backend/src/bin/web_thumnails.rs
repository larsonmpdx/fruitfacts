// walk through all .json5 references to find any marked "thumbnail: website"
// call out to /backend/web_screenshot/index.js to generate puppeteer screenshots for their reference urls
use std::fs;
use std::path::Path;
extern crate clap;
use clap::{crate_version, Arg, Command as ClapApp};

#[cfg(feature = "binaries")]
use pdfium_render::prelude::*;

#[cfg(feature = "binaries")]
fn web_address_to_png(web_address: &str, screenshot_path: &Path) -> Result<(), PdfiumError> {

    // todo - call out to web_screenshot/index.js

    Ok(())
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
            // todo - check if this reference has "thumbnail: website" (skip if it doesn't, maybe it has a pdf screenshot instead)

            // todo - fetch web address from the "url" field (and use only the first part before spaces)

            const web_address: &str = "todo";

            println!("loading reference: {}", input_path.display());

            let mut screenshot_path = input_path.to_path_buf();
            screenshot_path.set_extension("png");

            if screenshot_path.exists() && !matches.get_flag("redo_all") {
                println!("png already exists");
            } else if let Err(error) = web_address_to_png(web_address, &screenshot_path) {
                println!("pdfium error: {error:?}");
            }
        }
    }
}

#[cfg(not(feature = "binaries"))]
fn main() {
    // empty for when the feature isn't selected
    println!("feature \"binaries\" not selected, this tool was not built");
}
