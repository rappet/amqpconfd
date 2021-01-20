use crate::config::Config;
use std::path::PathBuf;
use tera::{Tera, Context};
use serde_json::Value;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::process::Command;
use log::warn;

#[derive(Debug)]
pub struct Templater {
    template_path: String,
    output_file: PathBuf,
    command: Option<String>,
}

impl Templater {
    pub fn new(
        template_path: String,
        output_file: PathBuf,
        command: Option<String>,
    ) -> Templater {
        Templater {
            template_path,
            output_file,
            command,
        }
    }

    pub fn from_config(config: &Config) -> Templater {
        Templater::new(
            config.template_path.to_string(),
            PathBuf::from(config.output_file.clone().into_owned()),
            config.apply_command.clone().map(|c| c.into_owned()),
        )
    }

    pub async fn apply_json(&self, json: &Value) -> anyhow::Result<()> {
        // read template
        let mut template_raw = Vec::new();
        File::open(&self.template_path).await?.read_to_end(&mut template_raw).await?;
        let template = String::from_utf8(template_raw)?;

        // Templating
        let context = Context::from_value(json.clone())?;
        let out = Tera::one_off(&template, &context, false)?;
        File::create(&self.output_file).await?.write_all(out.as_bytes()).await?;

        // Execute command
        if let Some(command) = &self.command {
            let status = Command::new("sh").arg("-c").arg(command).spawn()?.await?;
            if !status.success() {
                warn!("called process did not finish successfully");
            }
        }

        Ok(())
    }
}
