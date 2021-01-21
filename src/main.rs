extern crate anyhow;
extern crate clap;
extern crate env_logger;
extern crate futures_util;
extern crate lapin;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate tera;
extern crate tokio;
extern crate tokio_amqp;
extern crate toml;

use anyhow::Context;
use clap::Clap;
use log::{error, info};

use crate::config::Config;
use crate::consumer::ConfigChangeConsumer;
use crate::opts::Opts;
use crate::templater::Templater;
use std::sync::Arc;

mod config;
mod consumer;
mod opts;
mod templater;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    opts.init_logger();

    let config = Config::open(&opts.config).context("Failed to read the configuration")?;
    let templater = Arc::new(Templater::from_config(&config));

    info!("starting deamon");

    let mut consumer = ConfigChangeConsumer::connect(config.amqp_url.as_ref()).await?;
    consumer
        .consume(|value| {
            let templater = templater.clone();
            async move {
                templater.apply_json(&value).await
            }
        })
        .await?;

    error!("WIP unfinished, but here is the config: {:?}", &config);

    Ok(())
}
