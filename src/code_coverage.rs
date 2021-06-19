use std::{fs, marker::PhantomData};

use badge::{Badge, BadgeOptions};
use devx_cmd::{run, Cmd};

pub struct CodeCoverage<C: Config = DefaultConfig> {
    _config: PhantomData<C>,
}

pub trait Config {
    /// The entire cargo command after `cargo`.
    fn cargo_args() -> Vec<String> {
        vec![
            String::from("+nightly"),
            String::from("test"),
            String::from("--all-features"),
            String::from("--"),
            String::from("--nocapture"),
        ]
    }

    /// The list of paths ignored when calculating code coverage.
    fn ignore_paths() -> Vec<String> {
        Vec::default()
    }
}

impl<C: Config> CodeCoverage<C> {
    pub fn execute() -> anyhow::Result<()> {
        println!("Cleaning project");
        run!("cargo", "clean",)?;

        println!("Running tests");
        let mut cmd = Cmd::new("cargo");
        cmd.env("CARGO_INCREMENTAL", "0");
        cmd.env("LLVM_PROFILE_FILE", "%m.profraw");
        cmd.env("RUSTFLAGS", "-Zinstrument-coverage");
        cmd.args(C::cargo_args());
        cmd.run()?;

        println!("Generating coverage report");
        let mut cmd = Cmd::new("grcov");
        cmd.args(&[
            ".",
            "--binary-path",
            "./target/debug/",
            "-s",
            ".",
            "-t",
            "html",
            "--branch",
            "--ignore-not-existing",
            "--llvm",
            "-o",
            "coverage/",
            "--ignore",
            "target/*",
            "--ignore",
            "xtask/*",
        ]);
        for path in C::ignore_paths() {
            cmd.arg2("--ignore", path);
        }
        cmd.run()?;

        let coverage_percent = find_coverage_percent()?;
        let coverage_percent = format!("{:.02}%", coverage_percent);

        // Output with ::warning:: to display this message in github actions results.
        println!("::warning::Line Coverage Percentage: {}", coverage_percent);

        // Generate the coverage badge
        let svg = Badge::new(BadgeOptions {
            subject: String::from("coverage"),
            status: coverage_percent,
            color: String::from("#0366D6"),
        })
        .map_err(|message| anyhow::anyhow!(message))?
        .to_svg();
        fs::write("coverage/badge.svg", svg)?;

        println!("Cleaning up.");
        run!("find", ".", "-name", "*.profraw", "-exec", "rm", "{}", ";")?;

        Ok(())
    }
}

pub struct DefaultConfig;

impl Config for DefaultConfig {}

fn find_coverage_percent() -> anyhow::Result<f32> {
    let report = fs::read_to_string("coverage/index.html")?;
    assert!(!report.is_empty(), "coverage report was empty");

    let first_portion = report.split(" %").next().unwrap();
    let percentage = first_portion.split('>').last().unwrap();

    Ok(percentage.parse()?)
}