#!/bin/sh
set -ex

if [ $EUID -ne 0 ]; then
  echo "Please run as root"
  exit 1
fi

if [ -z "`command -v radvd`" ]; then
  echo "Please install radvd"
  exit 1
fi

if [ -z "`command -v dhcpd`" ]; then
  echo "Please install dhcpd"
  exit 1
fi

if [ -z "$1" ]; then
  echo "Please specify the interface device."
  echo "Usage: $0 <iface>"
  exit 1
fi

sysctl net.ipv6.conf.all.forwarding=1

ifaces=$(ip a | grep -P '^\d+: [\w\d]+:' | awk '{ print $2 }' | sed -r 's/(.*):$/\1/')

ip a a 2001:123:1:1::1 dev $1
ip r a 2001:123:1:1::/64 dev $1
ip a a 10.123.123.1 dev $1
ip r a 10.123.123.0/24 dev $1

if [ ! -f /etc/radvd.conf.backup ]; then
  mv /etc/radvd.conf{,.backup}
fi

cat > /etc/radvd.conf << _EOF_
interface $1
{
	AdvSendAdvert on;
	MinRtrAdvInterval 3;
	MaxRtrAdvInterval 15;

	prefix 2001:123:1:1::/64
	{
		AdvOnLink on;
		AdvAutonomous on;
		AdvRouterAddr off;
	};
};
_EOF_

if [ ! -f /etc/dhcpd.conf.example ]; then
  mv /etc/dhcpd.conf{,.example}
fi

cat > /etc/dhcpd.conf << _EOF_
option domain-name-servers 8.8.8.8, 8.8.4.4;
option subnet-mask 255.255.255.0;
option routers 10.123.123.1;
subnet 10.123.123.0 netmask 255.255.255.0 {
  range 10.123.123.20 10.123.123.100;
}
_EOF_

if [ -n "`command -v service`" ]; then
  service radvd start
  service dhcpd4 start
elif [ -n "`command -v systemctl`" ]; then
  systemctl start radvd
  systemctl start dhcpd4
fi

echo "Press return key to stop"
read

if [ -n "`command -v service`" ]; then
  service radvd stop
  service dhcpd4 stop
elif [ -n "`command -v systemctl`" ]; then
  systemctl stop radvd
  systemctl stop dhcpd4
fi
ip a del 2001:123:1:1::1 dev $1
ip r del 2001:123:1:1::/64 dev $1
ip a del 10.123.123.1 dev $1
ip r del 10.123.123.0/24 dev $1
