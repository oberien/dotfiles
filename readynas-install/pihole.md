# Pi-Hole (FTLDNS)

* ensure that `dnsmasq` is not installed - pihole-FTL replaces dnsmasq

Unbound:
```sh
apt install unbound dns-root-data dnsutils
# copy and edit configuration from:
# https://docs.pi-hole.net/guides/dns/unbound/#configure-unbound
vim /etc/unbound/unbound.conf.d/pi-hole.conf
systemctl enable unbound
systemctl restart unbound
# check if unbound works
dig google.de @127.0.0.1 -p 5335
dig dnssec.works @127.0.0.1 -p 5335 # NOERROR with IP
dig fail01.dnssec.works @127.0.0.1 -p 5335 # SERVFAIL without IP
```

Pi-Hole:
```sh
pushd /tmp
git clone --depth 1 https://github.com/pi-hole/pi-hole.git pi-hole
cd "pi-hole/automated install/"
./basic-install.sh
echo edns-packet-max=1232 > /etc/dnsmasq.d/99-edns.conf
cd ..
rm -r pi-hole

# add line `no-resolv`
vim /etc/dnsmasq.conf
# add DBINTERVAL=60.0
vim /etc/pihole/pihole-FTL.conf

systemctl enable pihole-FTL
systemclt start pihole-FTL

popd
```
