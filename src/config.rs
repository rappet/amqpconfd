use serde::{Serialize, Deserialize};

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    /// source path of the template to apply
    template_path: Cow<'a, str>,
    /// destination path of the templated file
    output_file: Cow<'a, str>,
    /// command to run after templating had been run
    apply_command: Option<Cow<'a, str>>,
    /// AMQP broker URL
    amqp_url: Cow<'a, str>,
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