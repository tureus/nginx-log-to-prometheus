FROM ubuntu:22.04

ADD target/release/nginx-log-to-prometheus /usr/local/bin/

CMD ["nginx-log-to-prometheus", "server"]