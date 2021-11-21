pub use anyhow;
pub use badge;
pub use devx_cmd;
pub use structopt;

pub mod audit;
pub mod code_coverage;
pub mod pre_commit;
use self::{audit::Audit, code_coverage::CodeCoverage};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Commands {
    /// Generates a code coverage report.
    GenerateCodeCoverageReport {
        #[structopt(long = "install-dependencies")]
        install_dependencies: bool,
    },
    /// Executes `cargo-deny`
    Audit { command: Option<String> },
    /// Installs the xtask binary as the pre-commit hook.
    InstallPreCommitHook,
}

pub fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::GenerateCodeCoverageReport {
            install_dependencies,
        } => CodeCoverage::<code_coverage::DefaultConfig>::execute(install_dependencies),
        Commands::Audit { command } => Audit::<audit::DefaultConfig>::execute(command),
        Commands::InstallPreCommitHook => pre_commit::install(),
    }
}
