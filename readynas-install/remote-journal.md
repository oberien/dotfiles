# Remote Journal

Make the systemd journal available on another server.

* systemd-journal-remote on the server
* systemd-journal-upload on the client
* uses HTTP (instead of HTTPS) as the servers have a direct wireguard connection


### On the server

```
apt install systemd-journal-remote

# remove the `s` from `https`
# ```
# [Service]
# ExecStart=
# ExecStart=/lib/systemd/systemd-journal-remote --listen-http=-3 --output=/var/log/journal/remote/
# ```
SYSTEMD_EDITOR=vim systemctl edit systemd-journal-remote

# make it listen only on the wireguard IP
# ```
# [Socket]
# ListenStream=
# ListenStream=<wireguard-server-ip>:19532
# ```
SYSTEMD_EDITOR=vim systemctl edit systemd-journal-remote

systemctl enable systemd-journal-remote
systemctl start systemd-journal-remote
# check if it started correctly
systemctl status systemd-journal-remote
```

On the Client:
```
apt install systemd-journal-remote

mkdir /etc/systemd/journal-upload.conf.d/
# ```
# [Upload]
# URL=http://<wireguard-server-ip>:19532
# ```
vim /etc/systemd/journal-upload.conf.d/url.conf

systemctl enable systemd-journal-upload
systemctl start systemd-journal-upload
# check if it started correctly
systemctl status systemd-journal-upload
```

Check the client-logs on the server:
```
journalctl --file=/var/log/journal/remote/*
```

