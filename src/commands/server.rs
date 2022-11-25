use crate::listener::Listener;
use clap::{Arg, ArgMatches, Command};
use tracing::info;

pub fn command() -> Command {
    Command::new("server").arg(
        Arg::new("bind")
            .long("bind")
            .short('b')
            .default_value("0.0.0.0:5445")
            .alias("listen"),
    )
}

pub async fn run(matches: &ArgMatches) -> eyre::Result<()> {
    let bind = matches.get_one::<String>("bind").unwrap();

    // let prometheus_registry = prometheus::Registry::default();
    // let metrics = prometheus_registry.gather();
    // info!("metrics: {:?}", metrics);

    let mut l = Listener::new(bind.to_owned(), |msg| {
        info!("msg: {:?}", msg);
    });
    l.run().await?;

    Ok(())
}
