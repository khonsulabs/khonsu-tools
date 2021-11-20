use std::{collections::HashSet, marker::PhantomData, path::Path, process::Stdio};

use devx_cmd::{Cmd, Ostream};
use serde::{Deserialize, Serialize};

pub struct Audit<C: Config = DefaultConfig> {
    _config: PhantomData<C>,
}

pub trait Config {
    /// The cargo command after `cargo`.
    fn args() -> Vec<String> {
        if Path::new("xtask").exists() {
            // Despite xtask being marked as publish = false, the dependencies are still checked.
            vec![
                String::from("--all-features"),
                String::from("--exclude=xtask"),
            ]
        } else {
            vec![String::from("--all-features")]
        }
    }
}

impl<C: Config> Audit<C> {
    pub fn execute(command: Option<String>) -> anyhow::Result<()> {
        let executing_on_ci = std::env::var("CI")
            .map(|ci| !ci.is_empty())
            .unwrap_or_default();
        let mut cmd = Cmd::new("cargo");
        cmd.arg("deny");

        if executing_on_ci {
            cmd.arg("--format=json");
        }

        cmd.args(C::args());
        cmd.arg("check");
        if let Some(command) = command {
            cmd.arg(command);
        }

        if executing_on_ci {
            let mut command = cmd.spawn_with(Stdio::inherit(), Stdio::piped())?;

            // Attempt to parse theoutput of the command, even if it failed.
            if let Ok(output) = command.read_no_wait(Ostream::StdErr) {
                for line in output.lines() {
                    if let Ok(parsed) = serde_json::from_str::<Report>(line) {
                        match parsed {
                            Report::Diagnostic(diagnostic) => {
                                // Without itertools: can't join on a HashSet . can't dedup in a Vec.
                                let graphs = diagnostic
                                    .graphs
                                    .iter()
                                    .map(|d| d.name.as_str())
                                    .collect::<HashSet<_>>();
                                let graphs = graphs.into_iter().collect::<Vec<_>>().join("; ");
                                println!(
                                    "::{}::{}: {}",
                                    diagnostic.severity, graphs, diagnostic.message
                                );
                                for label in diagnostic.labels {
                                    println!(
                                        "::{}::{}",
                                        diagnostic.severity,
                                        label.span.lines().collect::<Vec<_>>().join("; ")
                                    );
                                }
                            }
                            Report::Summary(_) => {}
                        }
                    } else {
                        eprintln!("{}", line);
                    }
                }
            }
            command.wait()?;
        } else {
            cmd.run()?;
        }

        Ok(())
    }
}

pub struct DefaultConfig;

impl Config for DefaultConfig {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "fields")]
#[serde(rename_all = "lowercase")]
enum Report {
    Diagnostic(Diagnostic),
    Summary(Advisories),
}

#[derive(Serialize, Deserialize, Debug)]
struct Diagnostic {
    code: String,
    graphs: Vec<Graph>,
    labels: Vec<Label>,
    message: String,
    severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Graph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Advisories {}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    column: usize,
    line: usize,
    message: String,
    span: String,
}
