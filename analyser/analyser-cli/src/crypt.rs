use std::io::Read;

use eyre::Result;
use ring::digest::{Context, Digest, SHA256};

pub trait DigestExt {
    fn to_str(&self) -> String;
}

pub fn digest_sha256<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }

        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
