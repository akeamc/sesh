use std::{fs::File, io::BufReader, path::Path};

use thiserror::Error;

use crate::{json::iter_json_array, Stream};

#[derive(Debug, Error)]
pub enum ReadZipError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub fn read_zip(
    path: impl AsRef<Path>,
    mut stream_cb: impl FnMut(Stream),
) -> Result<(), ReadZipError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(reader)?;

    // read all json files in the zip archive
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;

        if file.name().ends_with(".json") && file.name().contains("Audio") {
            for stream in iter_json_array(BufReader::new(file)) {
                stream_cb(stream?);
            }
        }
    }

    Ok(())
}

pub fn read_zip_to_end(path: impl AsRef<Path>) -> Result<Vec<Stream>, ReadZipError> {
    let mut out = Vec::new();
    read_zip(path, |stream| out.push(stream))?;
    Ok(out)
}
