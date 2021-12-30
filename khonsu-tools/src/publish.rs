use devx_cmd::Cmd;
use khonsu_universal_tools::DefaultConfig;

pub trait Config {
    fn paths() -> Vec<String>;
}

impl Config for DefaultConfig {
    fn paths() -> Vec<String> {
        Vec::new()
    }
}

pub fn execute<C: Config>(dry_run: bool, allow_dirty: bool) -> anyhow::Result<()> {
    let paths = C::paths();
    if paths.is_empty() {
        anyhow::bail!("no publish paths configured");
    }

    let starting_directory = std::env::current_dir()?;
    match rustme::generate_in_directory(&starting_directory, true) {
        Err(rustme::Error::NoConfiguration) => {}
        other => other?,
    }

    for path in paths {
        println!("Proceed with publishing {}? [y/yes]", path);
        check_continue()?;

        loop {
            std::env::set_current_dir(starting_directory.join(&path))?;
            let mut publish = Cmd::new("cargo");
            let publish = publish.arg("publish");
            if dry_run {
                publish.arg("--dry-run");
            }
            if allow_dirty {
                publish.arg("--allow-dirty");
            }
            if let Err(err) = publish.run() {
                eprintln!("Error encountered running the command: {}", err);
                eprintln!("Retry? [y/yes]");
                check_continue()?
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn check_continue() -> anyhow::Result<()> {
    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;
    if matches!(response.trim(), "y" | "yes" | "") {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Aborting."))
    }
}
