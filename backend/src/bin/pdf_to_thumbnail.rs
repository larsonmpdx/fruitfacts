// walk though all reference PDFs and create first page thumbnails of them
use std::fs;
use std::path::Path;

#[cfg(feature = "binaries")]
use pdfium_render::prelude::*;

#[cfg(feature = "binaries")]
fn pdf_first_page_to_jpeg(input_path: &Path, output_path: &Path) -> Result<(), PdfiumError> {
    // adapted from an example in the pdfium_render docs

    if output_path.exists() {
        println!("jpg already exists");
        return Ok(());
    }

    // Bind to a Pdfium library in the same directory as our Rust executable;
    // failing that, fall back to using a Pdfium library provided by the operating system

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file(input_path, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(1200)
        .set_maximum_height(1200);

    for (index, page) in document.pages().iter().enumerate() {
        if index != 0 {
            break;
        }

        let mut output = fs::File::create(output_path).map_err(|_| PdfiumError::ImageError)?;

        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .as_rgba8() // ... then converts it to an image::Image...
            .ok_or(PdfiumError::ImageError)?
            .write_to(&mut output, image::ImageOutputFormat::Jpeg(80)) // number is jpeg quality level
            .map_err(|_| PdfiumError::ImageError)?;

        output.sync_all().map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}

#[cfg(feature = "binaries")]
fn main() {
    // todo
    // find all *.pdf recursively in plant_database/
    // for any without an existing thumbnail, create one with the same path but changed extension

    let database_dir = harvest_chart_server::import_db::get_database_dir().unwrap();

    for entry in walkdir::WalkDir::new(database_dir.join("references"))
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path_ = entry.path();

        if fs::metadata(path_).unwrap().is_file() // filenames can't be >260 chars here without help - probably fixed in rust 1.58 - https://github.com/rust-lang/rust/issues/67403
        && path_.extension().unwrap().to_str().unwrap() == "pdf"
        {
            println!("loading reference: {}", path_.display());
            let mut mutable_path = path_.to_path_buf();
            mutable_path.set_extension("jpg");

            if let Err(error) = pdf_first_page_to_jpeg(path_, &mutable_path) {
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
