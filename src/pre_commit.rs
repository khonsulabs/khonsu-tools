use devx_pre_commit::locate_project_root;

pub fn install() -> anyhow::Result<()> {
    devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;

    Ok(())
}
