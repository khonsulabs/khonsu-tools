pub use khonsu_universal_tools as universal;

use structopt::StructOpt;
use universal::DefaultConfig;

pub mod publish;

#[derive(StructOpt, Debug)]
pub enum Commands {
    Readmes {
        #[structopt(long)]
        release: bool,
    },
    Publish {
        #[structopt(long)]
        dry_run: bool,
        #[structopt(long)]
        allow_dirty: bool,
    },
    #[structopt(flatten)]
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
    let command = Commands::from_args();
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
