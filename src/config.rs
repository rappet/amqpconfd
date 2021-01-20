use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config<'a> {
    /// source path of the template to apply
    pub template_path: Cow<'a, str>,
    /// destination path of the templated file
    pub output_file: Cow<'a, str>,
    /// command to run after templating had been run
    pub apply_command: Option<Cow<'a, str>>,
    /// AMQP broker URL
    pub amqp_url: Cow<'a, str>,
}

impl Config<'static> {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Config<'static>> {
        let mut content = Vec::new();
        File::open(path)?.read_to_end(&mut content)?;
        Ok(toml::from_slice(content.as_slice())?)
    }
}

impl Default for Config<'static> {
    fn default() -> Self {
        Config {
            template_path: "/etc/amqpconf.d/template.j2".into(),
            output_file: "/etc/amqpconf.d/output.txt".into(),
            apply_command: Some("cat /etc/amqpconf.d/output.txt".into()),
            amqp_url: "amqp://127.0.0.1:5672/%2f".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn default_config() {
        let config: Config = toml::from_str(include_str!("amqpconfd.toml")).unwrap();
        assert_eq!(Config::default(), config);
    }
}
