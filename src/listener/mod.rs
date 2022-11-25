use chrono::Datelike;
use syslog_loose::{parse_message_with_year_exact, Message};

pub struct Listener<F>
where
    F: FnMut(Message<&str>),
{
    uri: String,
    callback: F,
}

impl<F> Listener<F>
where
    F: FnMut(Message<&str>),
{
    pub fn new(uri: String, callback: F) -> Self {
        Listener { uri, callback }
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        let mut buf = [0; 1024 * 8];
        let listener = tokio::net::UdpSocket::bind(&self.uri).await?;

        loop {
            let bytes_read = listener.recv(&mut buf[..]).await?;
            let usable_buf = &buf[0..bytes_read];
            let str_buf = std::str::from_utf8(usable_buf)?;

            let parsed_message =
                parse_message_with_year_exact(str_buf, |_| chrono::Local::now().year())
                    .map_err(|e| eyre::eyre!(e))?;

            (self.callback)(parsed_message);
        }
    }
}
