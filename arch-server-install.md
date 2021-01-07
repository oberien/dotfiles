# Minimal (Arch) Server Configuration

## Install Arch Server
Install arch on the server machine you want to use.
For example for a RPi2 follow [this guide's installation tab](https://archlinuxarm.org/platforms/armv7/broadcom/raspberry-pi-2).

## Setup Network

By default `systemd-networkd` will be running.
It tries to get an IP via DHCP.
Check if you have internet with `ping 1.1` and for DNS check `ping google.com`.
If it works, and you have internet, you're in luck.

Otherwise, good luck debugging why you don't get a DHCP lease.
Key points here are to make sure that only one DHCP-daemon is running.
For example there are `systemd-networkd`, `dhcpcd`, `dhclient`, `NetworkManager` and more.
For testing, stopping all of the above on the machine and manually executing
`dhcpcd -t 0 -4` can help while debugging.
In the end, you'll probably end up just using a static IP via `systemd-networkd`
as described [here](https://wiki.archlinux.org/index.php/Systemd-networkd#Wired_adapter_using_a_static_IP).

## Getting root

If you log in as root by default, skip this.
If you log in as an unprivileged user and `sudo` isn't available, read on.
Instead of `sudo`, use `su` and type the password of the root account
(*not* the password of the currently logged in user).
That password is usually `root` or `toor`.
Refer to the installation image guide if those don't work.

## Pacman Update

If you haven't done so already, init the pacman keyring.
For example for ARM machines like the RPi, use the following:
```sh
pacman-key --init
pacman-key --populate archlinuxarm
```

Then perform a system update and install some tools you want (like sudo):
```sh
pacman -Syu sudo
```

## Hostname

Set the hostname to your liking by editing the file `/etc/hostname`.
Make sure to edit `/etc/hosts` accordingly by replacing the old hostname
with the new one in there (if the old hostname is used in there).

## User Management

If there is no unprivileged user yet (only root), you need to create one:
```sh
useradd -m -g users -G sudo,wheel,storage,power -s /bin/bash newusername
```

Otherwise, you can rename an existing user with the command below.
You must not be logged in as that user or have a session with that user in any way.
If you do, try logging in as root instead.
```sh
usermod -l newname oldname
```
If there is a group named after the user, make sure to rename that one as well
(you can see all existing groups with `cat /etc/group`):
```sh
groupmod -n newname oldname
```

Set the password of the new / renamed user.
After logging in as that user, run:
```sh
passwd
```

Allow users of the `wheel` group to use `sudo` by editing `/etc/sudoers`.
Uncomment the line `%wheel ALL=(ALL) ALL`.

Make sure that you can login as the new / renamed user.

Disable the root account from logging in in any way.
```sh
# lock the root account
passwd -l root
# scramble root password
usermod -p '!' root
# disable root login
usermod -s /sbin/nologin root
```

## SSH(d)

If you haven't already, create an ssh-key on your local machine and add it to
the server's user.
```sh
cd ~/.ssh
ssh-keygen -t ed25519 -f id_servername
ssh-copy-id -i id_servername username@ip
cat <<_EOF_ >> config
Host servername
    User username
    Hostname ip
    #Port 22
    IdentityFile ~/.ssh/id_servername
_EOF_
```

Edit the file `/etc/ssh/sshd_config` and add / uncomment / modify the following settings:
```
LogLevel VERBOSE
PermitRootLogin no
PasswordAuthentication no
```

Then restart `sshd` with `systemctl restart sshd`.

