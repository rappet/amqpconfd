use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Raphael Peters <rappet@rappet.de>")]
pub struct Opts {
    #[clap(short, long, env, default_value = "/etc/amqpconfd.toml")]
    #[clap(about = "Path to configuration file")]
    pub config: String,
    #[clap(short, long, parse(from_occurrences))]
    #[clap(about = "Level of verbosity (multiple)")]
    pub verbose: i32,
}

impl Opts {
    pub fn init_logger(&self) {
        match self.verbose {
            0 => {}
            1 => std::env::set_var("RUST_LOG", "amqpconfd=debug"),
            _ => std::env::set_var("RUST_LOG", "amqpconfd=trace"),
        }
        env_logger::init();
    }
}
