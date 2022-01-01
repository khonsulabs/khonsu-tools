pub use anyhow;
pub use badge;
pub use clap;
pub use devx_cmd;

pub mod audit;
pub mod code_coverage;
pub mod pre_commit;
use self::{audit::Audit, code_coverage::CodeCoverage};

use clap::Parser;

#[derive(Parser, Debug)]
pub enum Commands {
    /// Generates a code coverage report.
    GenerateCodeCoverageReport {
        #[clap(long = "install-dependencies")]
        install_dependencies: bool,
    },
    /// Executes `cargo-deny`
    Audit { command: Option<String> },
    /// Installs the xtask binary as the pre-commit hook.
    InstallPreCommitHook,
}

impl Commands {
    pub fn execute<C: Config>(self) -> anyhow::Result<()> {
        match self {
            Commands::GenerateCodeCoverageReport {
                install_dependencies,
            } => CodeCoverage::<C::CodeCoverage>::execute(install_dependencies),
            Commands::Audit { command } => Audit::<C::Audit>::execute(command),
            Commands::InstallPreCommitHook => pre_commit::install(),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let command = Commands::parse();
    command.execute::<DefaultConfig>()
}

pub trait Config {
    type Audit: audit::Config;
    type CodeCoverage: code_coverage::Config;
}

pub enum DefaultConfig {}

impl Config for DefaultConfig {
    type Audit = Self;
    type CodeCoverage = Self;
}
