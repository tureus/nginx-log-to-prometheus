use crate::listener::Listener;
use clap::{Arg, ArgMatches, Command};
use prometheus::core::{AtomicU64, GenericCounterVec};
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{debug, error, info};

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

    let bytes_sent_counter = prometheus::IntCounterVec::new(
        prometheus::Opts::new("bytes", "Bytes transferred out of upstreams"),
        &[
            "request_uri",
            "nginx_version",
            "host",
            "upstream_status",
            "software",
        ],
    )?;
    let duration_histogram = prometheus::Histogram::with_opts(prometheus::HistogramOpts::new(
        "duration_histogram",
        "Duration of requests to upstream servers",
    ))?;

    let prometheus_registry = prometheus::Registry::new_custom(Some("nginx".into()), None)?;
    prometheus_registry.register(Box::new(bytes_sent_counter.clone()))?;
    prometheus_registry.register(Box::new(duration_histogram.clone()))?;

    let mut exporter_server_builder =
        prometheus_exporter::Builder::new("0.0.0.0:9394".parse().unwrap());
    exporter_server_builder.with_registry(prometheus_registry);
    let _exporter = exporter_server_builder.start()?;
    info!("binding prometheus exporter to 0.0.0.0:9394");

    let mut l = Listener::new(bind.to_owned(), |msg| {
        info!(
            "facility: {:?}; severity: {:?}; hostname: {:?}; appname: {:?}; msg: {}",
            msg.facility, msg.severity, msg.hostname, msg.appname, msg.msg
        );

        let parsed_nginx_msg: Result<NGINXMessage, _> = serde_json::from_str(msg.msg);
        match parsed_nginx_msg {
            Ok(nginx_msg) => {
                update_bytes_sent(&bytes_sent_counter, &nginx_msg);
            }
            Err(e) => error!("failed to parse msg `{}`, err: `{}`", msg.msg, e),
        };
    });
    l.run().await?;

    Ok(())
}

fn update_bytes_sent(bytes_sent_counter: &GenericCounterVec<AtomicU64>, nginx_msg: &NGINXMessage) {
    let mut labels = HashMap::new();
    labels.insert("request_uri", nginx_msg.request_uri.as_str());
    labels.insert("software", nginx_msg.software.as_str());
    labels.insert("nginx_version", nginx_msg.nginx_version.as_str());
    labels.insert("host", nginx_msg.host.as_str());
    labels.insert("upstream_status", nginx_msg.upstream_status.as_str());

    match nginx_msg.body_bytes_sent.parse::<u64>() {
        Ok(bytes_sent) => {
            debug!("bumping {:?} by {}", labels, bytes_sent);
            bytes_sent_counter.with(&labels).inc_by(bytes_sent)
        }
        Err(e) => error!(
            "failed to parse `body_bytes_sent` {}, err: `{}`",
            nginx_msg.body_bytes_sent, e
        ),
    };
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NGINXMessage {
    pub time_local: String,
    pub remote_addr: String,
    pub remote_user: String,
    pub body_bytes_sent: String,
    pub request_time: String,
    pub upstream_header_time: String,
    pub status: String,
    pub request_uri: String,
    pub uri: String,
    pub args: String,
    pub request_method: String,
    pub http_referer: String,
    pub http_user_agent: String,
    pub software: String,
    pub nginx_version: String,
    pub host: String,
    pub upstream_addr: String,
    pub upstream_status: String,
    pub upstream_response_time: String,
}
