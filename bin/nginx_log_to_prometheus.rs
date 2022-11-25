use clap;
use tracing::Subscriber;
use tracing_subscriber::FmtSubscriber;

use nginx_log_to_prometheus_rs::commands;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    setup_tracing()?;

    let matches = clap_command().get_matches();

    match matches.subcommand() {
        Some(("server", matches)) => commands::server::run(matches).await?,
        Some(("client", matches)) => commands::client::run(matches).await?,
        Some((other, _)) => panic!("don't know how to handle {}", other),
        None => panic!("hey"),
    }

    Ok(())
}

pub fn clap_command() -> clap::Command {
    clap::Command::new("nginx_log_to_prometheus")
        .subcommand(commands::server::command())
        .subcommand(commands::client::command())
}

fn setup_tracing() -> eyre::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
