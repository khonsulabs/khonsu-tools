pub use anyhow;
pub use badge;
use code_coverage::CodeCoverage;
pub use devx_cmd;
pub use structopt;

pub mod code_coverage;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Commands {
    GenerateCodeCoverageReport,
}

pub fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::GenerateCodeCoverageReport =>
            CodeCoverage::<code_coverage::DefaultConfig>::execute(),
    }
}
