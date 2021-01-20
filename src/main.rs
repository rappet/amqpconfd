extern crate anyhow;
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate tera;
extern crate tokio;
extern crate toml;

use clap::Clap;
use log::{error, info};
use serde_json::json;
use anyhow::Context;

use crate::config::Config;
use crate::opts::Opts;
use crate::templater::Templater;

mod config;
mod opts;
mod templater;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    opts.init_logger();

    let config = Config::open(&opts.config).context("Failed to read the configuration")?;
    let templater = Templater::from_config(&config);

    templater.apply_json(&json!({
        "foo": "bar"
    })).await?;

    info!("starting deamon");

    error!("WIP unfinished, but here is the config: {:?}", &config);

    Ok(())
}
