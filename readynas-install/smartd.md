# Smartd

`/etc/smartd.conf`:
```
#...
DEVICESCAN -d removable -m root -s S/../../7/01 -M test -M daily -M exec /usr/share/smartmontools/smartd-runner
#...
```

Test sending a mail:
```
systemctl restart smartd
```

Disable test mail: In `/etc/smartd.conf` remove `-M test`

