FROM redhat/ubi8-micro

ADD target/release/nginx-log-to-prometheus /usr/local/bin/

CMD ["nginx-log-to-prometheus", "server"]