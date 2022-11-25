use clap::{Arg, ArgMatches, Command};
use tokio::net::UdpSocket;

pub fn command() -> Command {
    Command::new("client")
        .arg(
            Arg::new("dial")
                .long("dial")
                .short('d')
                .default_value("0.0.0.0:5445")
                .alias("dial"),
        )
        .arg(Arg::new("body").long("body").short('b').required(true))
        .arg(
            Arg::new("repetitions")
                .long("repetitions")
                .short('r')
                .default_value("1"),
        )
}

pub async fn run(matches: &ArgMatches) -> eyre::Result<()> {
    let dial = matches.get_one::<String>("dial").unwrap();
    let body = matches.get_one::<String>("body").unwrap();
    let repetitions = matches.get_one::<String>("repetitions").unwrap();
    let repetitions: usize = repetitions.parse()?;

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.connect(dial).await?;
    for _ in 0..repetitions {
        sock.send(body.as_bytes()).await?;
    }

    Ok(())
}
