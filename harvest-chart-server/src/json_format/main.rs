// format the json plant database
// adapted from this example tool: https://github.com/google/json5format/blob/master/examples/formatjson5.rs

use harvest_chart_server::import_db;
use json5format::*;
use std::fs;
use std::io;
use std::io::{Read, Write};
use walkdir::WalkDir;

fn write_to_file(filename: &str, bytes: &[u8]) -> Result<(), io::Error> {
    fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)?
        .write_all(bytes)
}

fn main() -> Result<(), std::io::Error> {
    let options = FormatOptions {
        indent_by: 4,
        trailing_commas: true,
        collapse_containers_of_one: true,
        sort_array_items: false,
        ..Default::default()
    };
    let format = Json5Format::with_options(options).unwrap();

    let database_dir = import_db::get_database_dir().unwrap();
    println!("found database dir: {:?}", database_dir);

    let file_paths = fs::read_dir(database_dir.join("plants")).unwrap();
    for file_path in file_paths {
        let path_ = file_path.unwrap().path();

        if fs::metadata(path_.clone()).unwrap().is_file()
            && path_.extension().unwrap().to_str().unwrap() == "json5"
        {
            run_format(path_, &format)?;
        }
    }

    for entry in WalkDir::new(database_dir.join("references"))
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path_ = entry.path();

        if fs::metadata(path_).unwrap().is_file()
            && path_.extension().unwrap().to_str().unwrap() == "json5"
        {
            run_format(path_.to_path_buf(), &format)?;
        }
    }

    Ok(())
}

fn run_format(path_: std::path::PathBuf, format: &Json5Format) -> Result<(), io::Error> {
    println!("formatting: {}", path_.display());
    let filename = path_.clone().into_os_string().to_string_lossy().to_string();
    let mut buffer = String::new();
    fs::File::open(&path_)?.read_to_string(&mut buffer)?;
    let parsed_document =
        ParsedDocument::from_string(buffer, Some(filename.clone())).unwrap();
    let bytes = format.to_utf8(&parsed_document).unwrap();
    write_to_file(&filename, &bytes)?;
    Ok(())
}
