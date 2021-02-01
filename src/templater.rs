use crate::config::Config;
use anyhow::Context;
use log::warn;
use serde_json::Value;
use std::path::PathBuf;
use tera::Tera;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug)]
pub struct Templater {
    template_path: String,
    output_file: PathBuf,
    /// command to run after appling config
    command: Option<String>,
}

impl Templater {
    pub fn new(template_path: String, output_file: PathBuf, command: Option<String>) -> Templater {
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
        File::open(&self.template_path)
            .await?
            .read_to_end(&mut template_raw)
            .await
            .context("Failed reading template file")?;
        let template =
            String::from_utf8(template_raw).context("Template file is not valid UTF-8")?;

        // Templating
        let context = tera::Context::from_value(json.clone())?;
        let out = Tera::one_off(&template, &context, false)
            .context("Template file is not valid Jinja2")?;
        File::create(&self.output_file)
            .await?
            .write_all(out.as_bytes())
            .await
            .context("Could not write the templated output")?;

        // Execute command
        if let Some(command) = &self.command {
            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .spawn()
                .context("Could not open the specified command")?
                .wait()
                .await
                .context("Specified command failed")?;
            if !status.success() {
                warn!("called process did not finish successfully");
            }
        }

        Ok(())
    }
}
