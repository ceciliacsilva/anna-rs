use anna::{anna_default_zenoh_prefix, config::Config, nodes::monitoring};
use argh::FromArgs;
use eyre::Context;
use std::{fs, path::PathBuf, sync::Arc};
use zenoh::prelude::sync::SyncResolve;

#[derive(FromArgs)]
/// Rusty anna monitor
struct Args {
    #[argh(positional)]
    config_file: PathBuf,
}

fn main() -> eyre::Result<()> {
    if let Err(err) = set_up_logger() {
        eprintln!(
            "{:?}",
            eyre::Error::new(err).wrap_err("failed to set up logger")
        );
    }

    let args: Args = argh::from_env();

    let config: Config = serde_yaml::from_str(
        &fs::read_to_string(args.config_file).context("failed to read config file")?,
    )
    .context("failed to parse config file")?;

    let zenoh = zenoh::open(zenoh::config::Config::default())
        .res()
        .map_err(|e| eyre::eyre!(e))?;
    let zenoh_prefix = anna_default_zenoh_prefix();

    monitoring::run(&config, Arc::new(zenoh), zenoh_prefix.to_owned())
}

fn set_up_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("monitor.log")?)
        .apply()?;
    Ok(())
}
