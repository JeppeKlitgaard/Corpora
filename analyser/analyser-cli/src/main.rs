#![feature(trait_alias)]

use eyre::Result;

mod cli;
mod http;
mod io;
mod analyse;
mod objects;
mod crypt;
mod utils;
mod occurance;
mod transforms;


fn main() -> Result<()> {
    cli::run()
}
