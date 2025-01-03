# nginx and Let's Encrypt

```
apt install nginx python3-certbot-nginx
```

* general nginx file structure:
    * `/etc/nginx/nginx.conf` contains global config which are the same for every server
    * `/etc/nginx/sites-available/` contains configs for each server respectively
    * `/etc/nginx/sites-enabled/` contains symlinks to `sites-available/...`
* test setup:
    * remove `sites-enabled/default`
    * set up test site under location `/test` partially following <https://www.digitalocean.com/community/tutorials/how-to-install-nginx-on-debian-10>
    * test config with `nginx -t`
    * apply with `systemctl reload nginx`
    * for connection reset: `return 444;`
    * set up server config following <https://nginx.org/en/docs/beginners_guide.html>
* let's encrypt + ssl:
    * set up ssl parameters following <https://wiki.mozilla.org/Security/Server_Side_TLS>
    * `certbot --nginx -d your_domain`
    * ensure proper configuration of `/etc/cron.d/certbot`


# nginx WebDAV

* on Windows, set `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\services\WebClient\Parameters\FileSizeLimitInBytes=0xFFFFFFFF` to increase file size limit from 50MB (default) to 4GiB

```nginx
# in case the distro requires it:
load_module /usr/lib/nginx/modules/ngx_http_dav_ext_module.so;
load_module /usr/lib/nginx/modules/ngx_http_headers_more_filter_module.so;
# on debian this is just: apt install libnginx-mod-http-dav-ext libnginx-mod-http-headers-more-filter

http {
    dav_ext_lock_zone zone=foo:10m;

    server {
        # Note: MUST NOT include trailing slash
        location /webdav {
            # alias is not supported, and there is no (easy) way to do rewrites
            # so this actually goes to /path/to/folder/webdav/
            root /path/to/folder/;

            # the easy part
            dav_methods PUT DELETE MKCOL COPY MOVE;
            dav_ext_methods PROPFIND OPTIONS LOCK UNLOCK;
            dav_ext_lock zone=foo;
            dav_access user:rw group:rw all:rw;

            # useful stuff
            client_max_body_size 0;
            send_timeout 3600;
            client_body_timeout 3600;
            keepalive_timeout 3600;
            lingering_timeout 3600;
            create_full_put_path on;

            # the hard parts
            if ($request_method = PROPPATCH) { # Unsupported, always return OK.
                add_header Content-Type 'text/xml';
                return 207 '<?xml version="1.0"?><a:multistatus xmlns:a="DAV:"><a:response><a:propstat><a:status>HTTP/1.1 200 OK</a:status></a:propstat></a:response></a:multistatus>';
            }

            # fixed version of https://www.robpeck.com/2020/06/making-webdav-actually-work-on-nginx/
            set $flags "";

            # check for COPY/MOVE request
            if ($request_method = MOVE) {
                set $flags "${flags}M";
            }
            if ($request_method = COPY) {
                set $flags "${flags}M";
            }

            # check for directory-targeting request: either targets
            # an existing directory, or uses MKCOL to create a new one
            if (-d $request_filename) {
                set $flags "${flags}D";
            }
            if ($request_method = MKCOL) {
                set $flags "${flags}D";
            }

            # check for missing trailing slash in Destination header
            if ($http_destination ~ [^/]$) {
                set $flags "${flags}R";
            }

            # for all directory-targeting requests, add the
            # (potentially missing) trailing slash
            if ($flags ~ "D") {
                rewrite ^(.*[^/])$ $1/;
            }

            # for
            # 1. a MOVE/COPY request ("M")
            # 2. targeting a directory ("D")
            # 3. where the Destination header is missing a trailing slash ("R")
            # we need to add the trailing slash to the Destination header
            if ($flags = "MDR") {
                more_set_input_headers "Destination: ${http_destination}/";
            }
            # important: the order is relevant
            # the rewrite must be placed before the more_set_input_headers
            # or else the latter just won't work. no idea why.


            # note for testing: windows will refuse to use basic auth over unencrypted http
            # you can change that in the registry:
            # HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\services\WebClient\Parameters\BasicAuthLevel=2
            auth_basic "webdav";
            auth_basic_user_file "/etc/nginx/.htpasswd-webdav";
            # the htpasswd file can use `<username>:{PLAIN}<password>`
            # e.g. `myuser:{PLAIN}asdfasdf`
        }
    }
}
```
