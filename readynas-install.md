# Install and Configure fresh Debian on ReadyNAS RN426

#### Setup Overview

Original ReadyNAS Software:
* kernel + compressed rootfs on internal flash drive connected directly to USB header on mainboard
* that usb device is configured to be the default EFI drive in BIOS
* during boot loads kernel and uncompresses rootfs in-memory
* when drives are added, it creates a 4 GB partition on them
* RAID1 over all those 4GB partition
* BTRFS in the raid device partition
* rootfs extracted into that partition

Disadvantages:
* all drives always spinning as there is always data that is read / logs written
* no ZFS support
* only X-Raid (which appears to be a shim over mdadm to automate some tasks)

Our Setup:
* have system on USB drives to allow HDDs to suspend
    * USB drives not made for many write cycles (no wear-leveling, bad flash)
    * -> use high endurance microSD cards instead
* USB-Hub in usb port
* 2 microSD cards (Samsung Pro Endurance 64GB) via MicroSD-USB-adapter in Hub
* raid1 boot partition on microSDs (512MB; ensure `--metadata=0.90`)
* raid1 system partition on microSDs (rest)
* ZFS over all HDDs
* minidlna for LAN media streaming
* CIFS / SMB shares for LAN storage access
* NextCloud for data sync and remote access / streaming

#### Install Debian

Prepare debian installation USB-drive:
* download small USB sticks image: <https://www.debian.org/distrib/netinst.en.html#smallcd>
* dd the iso over onto the usb drive (and sync)
* move over the usb drive and connect it to the NAS

Connect Serial Console:
* remove sticker from back of NAS from UART Port
* connect micro-USB to USB cable from NAS UART port to PC
* `screen /dev/ttyUSB0 115200`

Configure Bios:
* restart server
* over serial console you should see the boot codes
* then you should see the BIOS message
    * press ESC to open BIOS
* configure correct time
* disable CSM
* change boot device order to prefer your USB-drive
* feel free to change other things

Prepare system SD Cards:
* connect both SD cards to PC
* for both disks: `fdisk /dev/sdX`, `g;n,,,+512M,[y];n,,,,[y];t,1,1;t,2,raid;w`
* create md arrays:
```
mdadm --create /dev/md127 --metadata=0.90 --level=1 --raid-devices=2 /dev/sdX1 /dev/sdY1
mdadm --create /dev/md1 --level=1 --raid-devices=2 /dev/sdX2 /dev/sdY2
mkfs.fat -F32 /dev/md127
mkfs.ext4 /dev/md1
# wait until synchronization is finished (cat /proc/mdstat)
mdadm --stop /dev/md1
mdadm --stop /dev/md127
sync
```

Install Debian:
* connect SD cards to NAS
* in grub:
    * edit `Install`
    * add boot kernel parameter `console=ttyS0,115200n8`
* go through installation until partitioning
* when partitioning menu comes, switch to shell (Ctlr+a a n)
* ensure RAID is running (md1 and md127 exist in `cat /proc/partitions` or via `cat /proc/mdstat`)
    * otherwise assemble them
    * `mdadm --assemble /dev/md127 /dev/sdX1 /dev/sdY1`
    * `mdadm --assemble /dev/md1 /dev/sdX2 /dev/sdY2`
    * you may need to go back in the installer to `Detect Disks`
* use `md127` as EFI System Partition
* use `md1` as ext4, mount point `/`
* if installation fails (before grub install):
    * move to shell
    * umount everything
    * manually reformat the filesystems onto the raid devices
    * manually mount the system partition at `/target/`
    * manually mount the boot partition at `/target/boot/efi/`
    * back in the installer, run the step Install the base system
* if installation fails due to grub (<https://superuser.com/a/1316680>):
    * continue without bootloader
    * don't remove installation media
    * boot into Advanced option > Rescue mode (don't forget the kernel parameter)
    * from shell assemble raid devices
    * continue Rescue mode "installer" until you get a shell
    * mount `/boot/efi`
    * `dpkg-reconfigure grub-efi-amd64`
    * `apt install --reinstall grub-efi-amd64`
    * `efibootmgr -c -d /dev/sdc -L debian0 -l '\EFI\debian\grubx64.efi'`
    * `efibootmgr -c -d /dev/sdd -L debian1 -l '\EFI\debian\grubx64.efi'`
* update (or if needed manually create) the `/etc/fstab`
```
# / (/dev/md1)
UUID=... /               ext4    errors=panic 0       1
# /boot/efi (/dev/md127)
UUID=...  /boot/efi       vfat    errors=remount-ro,umask=0077      0       1
```
* edit `/etc/sysctl.conf`
    ```
    kernel.panic = 10
    ```
* update debian version
    * edit `/etc/apt/sources.list` and replace your current version with the new version
    * `apt update`
    * `apt dist-upgrade` (possibly multiple times)
        * if there are gpg errors, do:
        * `apt install gnupg`
        * `apt-key adv --keyserver hkp://pool.sks-keyservers.net:80 --recv-keys <key>`
    * `apt autoremove`
* in BIOS in Boot options, disable everything except the two RAID1 boot partition entries
    * also disable the UEFI console, as it'll run anyway one no other boot device was found

#### Set up server, ssh etc

* follows relevant parts of <https://github.com/oberien/dotfiles/blob/master/arch-server-install.md>

#### Install Fan Driver

This is only needed if you're using the stock fan.
If you exchanged the stock 120mm fan with e.g. a Noctua (mind the different FAN
header pinning that netgear uses), the fan should be silent enough for the
default BIOS fan control to work perfectly fine.

```
# if not already installed, install linux-headers
#apt install linux-headers
apt install git make linux-headers-amd64 dkms lm-sensors fancontrol
git clone https://github.com/a1wong/it87
cd it87
make clean
make
make dkms
modprobe it87
sensors-detect #type YES everywhere
pwmconfig
systemctl enable fancontrol
systemctl start fancontrol
```

#### Enable HDD Spindown

To send HDDs to sleep after 5min, in `/etc/hdparm.conf` for each HDD add
```
/dev/disk/by-id/... {
	spindown_time = 60
}
```

#### Configure Network

Single LAN connection (`/etc/network/interfaces`):
```
source /etc/network/interfaces.d/*

auto lo
iface lo inet loopback

allow-hotplug enp2s0f0
allow-hotplug enp2s0f1
allow-hotplug enp3s0f0
allow-hotplug enp3s0f1
iface enp2s0f0 inet manual
iface enp2s0f1 inet manual
iface enp3s0f0 inet dhcp
iface enp3s0f1 inet manual
```

LAG with LACP (requires LAG / LACP supporting switch) (`/etc/network/interfaces`):
```
source /etc/network/interfaces.d/*

auto lo
iface lo inet loopback

# Link Aggregation / LAG / 802.3ad
auto bond0

iface bond0 inet dhcp
    bond-slaves enp2s0f0 enp2s0f1 enp3s0f0 enp3s0f1
    bond-miimon 100
    bond-downdelay 200
    bond-updelay 200
```
reboot

#### Setup ZFS

```
# if not already installed, install linux-headers
#apt install linux-headers
apt install zfs-dkms
ls /dev/disk/by-id/
zpool create -O mountpoint=none tank raidz2 <hdd-ids...> cache <ssd-id>
# check ashift
zdb -C | grep ashift
dd if=/dev/urandom of=/keys/keyfile_zfs bs=1 count=32
zfs create -o encryption=aes-256-gcm -o keyformat=raw -o keylocation=file:///keys/keyfile_zfs -o atime=off -o compression=lz4 -o dedup=on -o mountpoint=/data tank/data
# for already compressed files like movies, images or music
zfs create -o encryption=aes-256-gcm -o keyformat=raw -o keylocation=file:///keys/keyfile_zfs -o atime=off -o compression=zle -o mountpoint=/data tank/data
```

Check that trim / scrub is executed regularly:
```
# change hour to 2 to execute trim/scrub every first/second sunday at 2:24am
vim /etc/cron.d/zfsutils-linux
systemctl status cron
```

Decrypt after restart:
```
zfs load-key -a
zfs mount tank/data
# if using minidlna
systemctl start minidlna
```

SMB (no CI) (fast)
```
apt install samba
# disable sharing homes by commenting out the `[homes]` section
vim /etc/samba/smb.conf
zfs set sharesmb=on tank/data
zfs share tank/data
smbpasswd -a root
# test if sharing worked
smbclient -U guest -N -L localhost

# mount on client
pacman -S samba
mount -t cifs //10.x.x.x/tank_data /data -o user=root,password=...,gid=1000,uid=1000,_netdev,x-systemd.automount,x-systemd.mount-timeout=1min
```

NFS (no CIA) (slow)
```
apt install nfs-kernel-server
zfs set sharenfs="rw,no_root_squash" tank/data
sysctl sunrpc.tcp_slot_table_entries=128

# mount on client
pacman -S nfs-utils
mount -t nfs4 10.x.x.x:/data /data
```

#### Backup (to external HDD)
Prepare backup HDD (only once initially)
```
# with reencryption
zpool create -O mountpoint=none -O encryption=aes-256-gcm -O keyformat=passphrase -O pbkdf2iters=10000000 backup /dev/disk/by-id/... #external hdd
# without reencryption
zpool create -O mountpoint=none backup /dev/disk/by-id/... #external hdd
```
Helpful commands
```
# list encryption status
zfs list -o name,encryption,keystatus,keyformat,keylocation,encryptionroot
# list all snapshots
zfs list -t snapshot
# import all pools
zpool import -a
# list zfs label
zdb -l /dev/sdx1
# import a pool which can't be imported automatically
zfs import -a -d /dev/disk/by-id/...
# upgrade an old pool
zfs upgrade <pool>
# decrypt if using reencryption
zfs load-key backup
```
Full backup
```
zfs snapshot -r tank/data@YYYY-MM-DD
# with reencryption - needs to be done manually for all snapshots
zfs send -cv tank/data@YYYY-MM-DD | zfs receive backup/data@YYYY-MM-DD
# without reencryption
zfs send -Rcwv tank/data@YYYY-MM-DD | zfs receive -F backup/data
```

#### Backup (to remote ZFS pool)

#### DLNA

```
apt install minidlna
# set media directory: media_dir=/foo
vim /etc/minidlna.conf
# only enable if not using encryption
systemctl enable minidlna
systemctl start minidlna
```

#### nginx and Let's Encrypt

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

#### nginx WebDAV

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

#### Pi-Hole (FTLDNS)

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

#### MQTT

```
apt install mosquitto mosquitto-clients
# add the following things:
# allow_anonymous true
# listener 1883
vim /etc/mosquitto/mosquitto.conf
systemctl restart mosquitto
# test
mosquitto_sub -t 'test/#' -v
mosquitto_pub -t 'test/foo' -m 'lol'
```

#### Nextcloud

```
apt install docker-compose
```

**Don't use this:** (uses nextcloud+apache)

* add apt repository to `/etc/apt/sources.list` from
  <https://www.jurisic.org/index.php?pages/My-Debian-Repository>
* add keyring as described on the same page

```
apt update && apt upgrade
apt install nextcloud-server
```

#### Email Notifications

##### Setup E-Mail client

Setup `msmtp` (and `mail` to use `msmtp`):
```
apt install msmtp msmtp-mta bsd-mailx
apt purge mailutils
```
`/etc/msmtprc`:
```
defaults
auth on
tls on
tls_trust_file /etc/ssl/certs/ca-certificates.crt
syslog LOG_MAIL
aliases /etc/aliases

account gmx
host mail.gmx.net
port 587
from foo@gmx.de
user foo@gmx.de
passwordeval "cat /root/.ssh/msmtp-gmx-password"
set_from_header on
undisclosed_recipients on

account default : gmx
```
`/etc/aliases`:
```
#...
default: foo@gmx.de
```

Test if everything works:
```
# test sending directly via msmtp
printf "Subject: Test\n\nHello World" | msmtp foo@gmx.de
# test sending via mail
mail -s Testmail foo@gmx.de <<< test
```

##### ZFS Event Daemon (ZED)

Ensure the following config parameters in `/etc/zfs/zed.d/zed.rc`:
```
ZED_EMAIL_ADDR="root"
ZED_NOTIFY_VERBOSE=1
ZED_NOTIFY_DATA=1
ZED_SYSLOG_TAG="zed"
```

Modify `/etc/zfs/zed.d/statechange-notify.sh` with the changes in
<https://github.com/cbane/zfs/commit/f4f16389413061ed0b670df1cbd17954518a3096>.

Ensure zed is running:
```
systemctl status zed
systemctl restart zed
# if it wasn't started automatically:
systemctl enable zed
```

Test sending a mail:
```
truncate -s 512M /dev/shm/test
zpool create test /dev/shm/test
zpool scrub test
# email should be received now
zpool destroy test
rm /dev/shm/test
```

##### Smartd

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

#### Automatic updates (`cron-apt`)

```
apt install cron-apt
```

`/etc/cron-apt/config`:
```
MAILTO="foo@gmx.de"
MAILON="upgrade" # always, upgrade, error
```

`/etc/cron-apt/action.d/3-download`:
remove `-d` (download-only) flag

check `/etc/cron.d/cron-apt`

test cron-apt from cli: `cron-apt`


#### Manually Compile Kernel

On the readynas (taken from <https://kernel-team.pages.debian.net/kernel-handbook/ch-common-tasks.html#s-common-official>):
```
apt install build-essential fakeroot
apt build-dep linux
# if you have more than 20GB RAM, build linux in ramfs
mount -o remount,size=30G,noatime /dev/shm
cd /dev/shm
apt source linux
cd linux-x.y.z/
# make any changes you want
cp /boot/config-* .config
# set CONFIG_SYSTEM_TRUSTED_KEYS=""
vim .config
make oldconfig
make -j`nproc` bindeb-pkg 
cd ..
mv *.deb ~/
cd ~/
dpkg -i ./linux-image-x.y.z_x.y.z.deb ./linux-headers-x.y.z_x.y.z.deb
# after reboot, purge the old linux images including the old linux-image-x.y.z-amd64
apt purge linux-image-amd64 linux-image-...
```

#### System Benchmarks

```
# package linuxperf
perf bench mem memcpy --size 16GB
perf bench mem memset --size 16GB
# test sync performance (from postgresql package)
pg_test_fsync
# test filesystem performance (in-cache; for on-hdd remove parameter)
bonnie++
# test sequential read / write
dd if=/dev/zero of=/foo/fo bs=1M count=10k
# clear cache in between, e.g. via `zpool export tank && zpool import tank`
dd if=/foo/fo of=/dev/null bs=1M
# test random io performance
fio --name=test --size=10G --readwrite=read --directory=/test --time_based --runtime=100000
```

Network benchmark:

```
# on server
iperf -s
# on client
iperf -d -c <server ip>
```

#### ~~Intel QAT~~

**NONE OF THE BELOW WORKS**

Intel QAT firmware:

```
# add non-free property to all sources
vim /etc/apt/sources.list
apt update
apt install firmware-misc-nonfree
reboot
# check that intel_qat is loaded
dmesg | grep qat
lsmod | grep qat
```

Intel QAT Driver:
* download latest "Intel QuickAssist Technology Driver for Linux" from
  <https://01.org/intel-quickassist-technology>
* copy over to readynas

```
mkdir qat-driver && cd qat-driver
tar xzf qat*.tar.gz
# follow README, below are commands for QAT 1.7.l.4.12.0-00011
apt install libboost-dev libudev-dev build-essential pkg-config
CXXFLAGS=-march=native CFLAGS=-march=native ./configure --enable-qat-lkcf --enable-kapi --enable-qat-coexistence
CXXFLAGS=-mcx16 CFLAGS=-mcx16 CPPFLAGS=-mcx16 ./configure --enable-qat-lkcf --enable-kapi
make -j4
rmmod qat_c3xxx usdm_drv intel_qat
make install
cp build/qat_c3xxx.ko /usr/lib/modules/4.19.0-13-amd64/kernel/drivers/crypto/qat/qat_c3xxx/qat_c3xxx.ko
cp build/qat_c3xxx.ko /usr/lib/modules/4.19.0-13-amd64/updates/drivers/crypto/qat/qat_c3xxx/qat_c3xxx.ko
cp build/intel_qat.ko /usr/lib/modules/4.19.0-13-amd64/kernel/drivers/crypto/qat/qat_common/intel_qat.ko
cp build/intel_qat.ko /usr/lib/modules/4.19.0-13-amd64/updates/drivers/crypto/qat/qat_common/intel_qat.ko

# test
apt install libssl-dev zlib1g-dev
make samples
```

Intel QAT Engine:

```
git clone https://github.com/intel/QAT_Engine.git
apt install autogen autoconf libtool
./autoconf.sh
./configure --with-qat_dir=../qat-driver/ --with-openssl_install_dir=/opt
make -j4
make install
cp /opt/lib/engines-1.1/qatengine.so /opt/lib/engines-1.1/qatengine.la /usr/lib/x86_64-linux-gnu/engines-1.1
```
