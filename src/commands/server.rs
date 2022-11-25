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

    let backend_counter = prometheus::Counter::new("backend", "backend")?;

    let prometheus_registry = prometheus::Registry::default();
    prometheus_registry.register(Box::new(backend_counter.clone()))?;

    let mut l = Listener::new(bind.to_owned(), |msg| {
        info!(
            "facility: {:?}; severity: {:?}; hostname: {:?}; appname: {:?}; msg: {}",
            msg.facility, msg.severity, msg.hostname, msg.appname, msg.msg
        );
    });
    l.run().await?;

    Ok(())
}
