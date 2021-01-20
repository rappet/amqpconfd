extern crate anyhow;
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate tokio;
extern crate serde;
extern crate serde_json;
extern crate toml;

use clap::Clap;
use log::{info, warn};

use crate::opts::Opts;

mod opts;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    opts.init_logger();

    info!("starting deamon");

    error!("WIP unfinished");

    Ok(())
}
