pub use khonsu_universal_tools as universal;

use clap::Parser;
use universal::DefaultConfig;

pub mod publish;

#[derive(Parser, Debug)]
pub enum Commands {
    Readmes {
        #[clap(long)]
        release: bool,
    },
    Publish {
        #[clap(long)]
        dry_run: bool,
        #[clap(long)]
        allow_dirty: bool,
    },
    #[clap(flatten)]
    Universal(universal::Commands),
}

impl Commands {
    pub fn execute<C: Config>(self) -> anyhow::Result<()> {
        match self {
            Commands::Universal(command) => command.execute::<C::Universal>(),
            Commands::Readmes { release } => rustme::generate(release).map_err(anyhow::Error::from),
            Commands::Publish {
                dry_run,
                allow_dirty,
            } => publish::execute::<C::Publish>(dry_run, allow_dirty),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let command = Commands::parse();
    command.execute::<DefaultConfig>()
}

pub trait Config {
    type Publish: publish::Config;
    type Universal: universal::Config;
}

impl Config for DefaultConfig {
    type Publish = Self;
    type Universal = Self;
}
