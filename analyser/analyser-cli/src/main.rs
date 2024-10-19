#![feature(trait_alias)]

use eyre::Result;

mod analyse;
mod cli;
mod crypt;
mod http;
mod io;
mod objects;
mod occurance;
mod transforms;
mod utils;

fn main() -> Result<()> {
    cli::run()
}
