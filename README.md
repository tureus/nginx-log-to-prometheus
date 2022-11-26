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

Configuring NGINX
---

NGINX will need a stanza for writing out syslog and it should send out an info-rich JSON format. I use the [nginx-inc/nginx-stable helm chart](https://helm.nginx.com/) and this value gets me going:

    controller:
      config:
        entries:
          http-snippets: |
            log_format json_log '{\"time_local\": \"$time_local\", \"remote_addr\": \"$remote_addr\", \"remote_user\": \"$remote_user\", \"body_bytes_sent\": \"$body_bytes_sent\", \"request_time\": \"$request_time\", \"upstream_header_time\": \"$upstream_header_time\", \"status\": \"$status\", \"request_uri\": \"$request_uri\", \"uri\": \"$uri\", \"args\": \"$args\", \"request_method\":\"$request_method\", \"http_referer\": \"$http_referer\", \"http_user_agent\": \"$http_user_agent\", \"software\": \"nginx\", \"nginx_version\": \"$nginx_version\", \"host\": \"$host\", \"upstream_addr\": \"$upstream_addr\", \"upstream_status\": \"$upstream_status\", \"upstream_response_time\": \"$upstream_response_time\", \"proxy_add_x_forwarded_for\": \"$proxy_add_x_forwarded_for\"}';
            access_log syslog:server=127.0.0.1:5531,facility=local7,tag=nginx,severity=info json_log;
      extraContainers:
        - name: exporter
          image: ghcr.io/tureus/nginx-log-to-prometheus:master
          imagePullPolicy: Always
          args: ["nginx-log-to-prometheus", "server", "--bind=0.0.0.0:5531"]
          ports:
            - name: nginx-log-http
              containerPort: 9394

which in my setup renders a `/etc/nginx.conf` in the nginx-ingress container that looks something like:

    nginx@nginx-nginx-ingress-78c59c5576-4rsnw:/$ cat /etc/nginx/nginx.conf 
    worker_processes  auto;
    daemon off;
    
    error_log  stderr notice;
    pid        /var/lib/nginx/nginx.pid;
    
    events {
        worker_connections  1024;
    }
    
    http {
        include       /etc/nginx/mime.types;
        default_type  application/octet-stream;
        
        log_format json_log '{\"time_local\": \"$time_local\", \"remote_addr\": \"$remote_addr\", \"remote_user\": \"$remote_user\", \"body_bytes_sent\": \"$body_bytes_sent\", \"request_time\": \"$request_time\", \"upstream_header_time\": \"$upstream_header_time\", \"status\": \"$status\", \"request_uri\": \"$request_uri\", \"uri\": \"$uri\", \"args\": \"$args\", \"request_method\":\"$request_method\", \"http_referer\": \"$http_referer\", \"http_user_agent\": \"$http_user_agent\", \"software\": \"nginx\", \"nginx_version\": \"$nginx_version\", \"host\": \"$host\", \"upstream_addr\": \"$upstream_addr\", \"upstream_status\": \"$upstream_status\", \"upstream_response_time\": \"$upstream_response_time\", \"proxy_add_x_forwarded_for\": \"$proxy_add_x_forwarded_for\"}';
        access_log syslog:server=127.0.0.1:5531,facility=local7,tag=nginx,severity=info json_log;

        # SNIP
    }