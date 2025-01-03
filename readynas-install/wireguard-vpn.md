# Wireguard VPN

#### Locally on the client:
```
pacman -S wireguard-tools wireguard-vanity-address wireguard-arch
# generate client-key-pair and server-key-pair
wireguard-vanity-address <clientname>
wireguard-vanity-address <servername>
# generate pre-shared key for connection
wg genpsk
```
create file `/etc/wireguard/wg0.conf` (chown to root, chmod to 600):
```
[Interface]
Address = 10.166.39.2/32
PrivateKey = <client-private-key>
# public key: <client-public-key>

# if used as a hop / gateway instead of as a simple client, uncomment
#PreUp = iptables -P FORWARD DROP
#PreUp = sysctl -w net.ipv4.ip_forward=1
#PreUp = sysctl -w net.ipv4.conf.all.forwarding=1
#PostUp = iptables -A FORWARD -i %i -j ACCEPT
#PostUp = iptables -A FORWARD -o %i -j ACCEPT
#PostUp = iptables -t nat -A POSTROUTING --dest 192.168.178.0/24 -j MASQUERADE
#PostDown = iptables -D FORWARD -i %i -j ACCEPT
#PostDown = iptables -D FORWARD -o %i -j ACCEPT
#PostDown = iptables -t nat -D POSTROUTING --dest 192.168.178.0/24 -j MASQUERADE


[Peer]
PublicKey = <server-public-key>
PresharedKey = <client-server-psk>
Endpoint = my-ddns.domain.to.server:666
AllowedIPs = 192.168.178.0/24
```

#### On the server:

Verify IP Forwarding is enabled:
* `sysctl -a | grep 'net.ipv4.ip_forward =\|net.ipv4.conf.all.forwarding ='`
* if both show ` = 1`, it's fine
* otherwise, follow <https://wiki.archlinux.org/title/Internet_sharing#Enable_packet_forwarding>

create file `/etc/wireguard/wg0.conf` (chown to root, chmod to 600):
```
[Interface]
Address = 10.166.39.1/32
PrivateKey = <server-private-key>
# public key: <server-public-key>
ListenPort = 51820

PreUp = iptables -P FORWARD DROP
PreUp = sysctl -w net.ipv4.ip_forward=1
PreUp = sysctl -w net.ipv4.conf.all.forwarding=1

PostUp = iptables -A FORWARD -i %i -j ACCEPT
PostUp = iptables -A FORWARD -o %i -j ACCEPT
PostUp = iptables -t nat -A POSTROUTING -o <eth-device> -j MASQUERADE
PostDown = iptables -D FORWARD -i %i -j ACCEPT
PostDown = iptables -D FORWARD -o %i -j ACCEPT
PostDown = iptables -t nat -D POSTROUTING -o <eth-device> -j MASQUERADE

[Peer]
PublicKey = <client-public-key>
PresharedKey = <client-server-psk>
AllowedIPs = 10.166.39.2/32
```
