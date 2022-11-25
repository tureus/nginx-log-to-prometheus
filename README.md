NGINX Log To Prometheus
---

Expose your NGINX access logs as easy-to-scrape prometheus stats. NGINX can forward access logs over udp syslog (rfc3164).

Benefits:

 * Small executable, fast execution
 * Easy to read codebase
 * Not much to configure, not a general purpose syslog-to-prometheus convert. Hardcoded for NGINX access logs.

Testing it out
---

    docker run --platform=linux/amd64 --rm -it --net=host -e RUST_LOG=info ghcr.io/tureus/nginx-log-to-prometheus:master

then in another session

    docker run --platform=linux/amd64 --rm -it --net=host -e RUST_LOG=info ghcr.io/tureus/nginx-log-to-prometheus:master client --body='<34>2020-10-11T22:14:15.00Z mymachine app[323]: a message'

Prior art
---

  * https://github.com/martin-helmich/prometheus-nginxlog-exporter