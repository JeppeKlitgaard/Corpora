use std::{fs::File, io::{Write, BufReader, BufWriter, BufRead}, path::{PathBuf, Path}};

use eyre::{eyre, Result};
use flate2::read::GzDecoder;
use reqwest::{self, header::CONTENT_TYPE};
use tar::Archive;
use url::Url;

fn get_response(url: Url) -> Result<reqwest::blocking::Response> {
    let resp = reqwest::blocking::get(url.clone())?;

    let status = resp.status();
    if !resp.status().is_success() {
        return Err(eyre!("Got bad response: {status}"));
    }

    Ok(resp)
}

#[allow(unused)]
pub fn download_file(url: Url, out_path: PathBuf) -> Result<()> {
    let resp = get_response(url)?;
    let content = resp.bytes()?;
    let mut file = File::create(out_path.clone())?;

    _ = file.write_all(&content)?;

    Ok(())
}

pub fn download_and_decompress_archive(url: Url, out_path: &Path) -> Result<()> {
    let resp = get_response(url)?;
    let headers = resp.headers();

    let content_type = headers
        .get(CONTENT_TYPE)
        .ok_or_else(|| eyre!("Response contained no content type!"))?
        .to_str()?
        .to_owned();

    let content = &resp.bytes()?;

    // println!("{}", content_type);
    // let content_type = Mime::from_str(raw_content_type.to_str()?)?;
    match content_type.as_str() {
        "application/x-gzip" => {
            let decoded = GzDecoder::new(&content[..]);
            let mut archive = Archive::new(decoded);

            let sentences_entry = archive
                .entries()?
                .find_map(|e| {
                    let entry = e.ok()?;
                    let entry_path = entry.path().ok()?;
                    let filename = entry_path.file_name()?;

                    if filename.to_string_lossy().ends_with("-sentences.txt") {
                        return Some(entry);
                    }

                    None
                })
                .ok_or_else(|| eyre!("Could not find sentences file!"))?;

            let file = File::create(out_path)?;

            let mut data_reader = BufReader::new(sentences_entry);
            let mut data_writer = BufWriter::new(file);

            let mut length = 1;
            while length > 0 {
                let buffer = data_reader.fill_buf()?;

                let _ = data_writer.write(buffer);

                length = buffer.len();
                data_reader.consume(length);
            }
        }

        _ => {
            return Err(eyre!("Bad MIME type: '{}'", content_type));
        }
    }
    Ok(())
}
