# Switch to ZFS rootfs

Disadvantages of 2-disk RAID1:
* full partition is synced, even if only few GBs are used
* induced faults can't be recognized
* no quorum which block is broken if a block differs between both devices
* no nice scrub

Advantages of ZFS Mirror:
* checksums ensure corrupted blocks are noticed
* checksums allow detection of broken / healthy block
* checksums allow nice scrubbing
* easy snapshots / backups

Disadvantages of ZFS (for system partition):
* high fragmentation; usually <70% of space usable
    * for small system partition ~20GB of 120GB usable space fine
    * for small system partition backup-and-restore defragment procedure can be done very fast
* weird grub2 setup recommended by openzfs (arch wiki argues against that setup)

(Boot-)Setup:
* 512MB `/boot/efi` EFI FAT32 partition with BLSs and systemd-boot
* rest `/` ZFS root partition
* [openzfs wiki](https://openzfs.github.io/openzfs-docs/Getting%20Started/Debian/Debian%20Bookworm%20Root%20on%20ZFS.html) recommends splitting grub2 EFI FAT32 `/boot/efi` and grub2-compatible-ZFS `/boot`
    * grub2 ZFS implementation only allows few ZFS features
    * grub2 ZFS implementation ["is not known for reliability."](https://wiki.archlinux.org/title/Install_Arch_Linux_on_ZFS#Using_GRUB2)
    * all files recursively in /boot can easily be regenerated and thus are not integral for a full-state backup
        * reinstall all packets that put stuff in `/boot` (can be listed via `dpkg -S /boot`)
        * reinstall bootloader (grub2 / systemd-boot) to `/boot/efi`
        * optional: run `update-initramfs` (already done when reinstalling the linux kernel with DKMS modules presents, which is the case due to openzfs)
    * grub will still be non-ZFS, meaning the boot process is still not fully protected by checksums
    * -> don't use grub; use systemd-boot
    * -> don't use 3 partitions; have one `/boot/efi`-partition and one `/`-root-partition
* grub adds quite some complexity without any gain when having vmlinuz and grub on the same partition
* easiest solution would be a single vmlinuz and initramfs without a bootloader, loaded directly using vmlinuz's efistub
* but: Debian supports having and booting multiple different kernels
    * by default `linux-image-rt-amd64` installs the newest version and keeps around the last 2 versions as well
    * thus there are 3 vmlinuz and 3 initramfs in `/boot`, all with their respective versions
    * grub lists all vmlinuz files matching `/boot/vmlinuz-* /vmlinuz-* /boot/kernel-*`
    * grub uses `version_find_latest` to set the default version to boot
* thus we need a way to have a bootloader autodetect and boot the latest kernel version, with a fallback if it doesn't boot
* systemd-boot follows the [Boot Loader Specification](https://uapi-group.org/specifications/specs/boot_loader_specification/#sorting)
* it loads Type#1 entries (manually configured or via BLS hooks) from `loader/entries` -> we'll use this
* if loads Type#2 entries (UKIs) from `esp/EFI/Linux/*.efi`
* Type#2 entries are sorted based on their filenames using descending version sort -> newest version will be loaded by default
* UKIs aren't supported by Debian bookworm
    * bookworm-backports contains systemd-ukify as part of systemd; installing systemd/bookworm-backports uninstalls everything incompatible with new systemd (including mdadm and lots of other integral parts)
    * manual backport of systemd-ukify requires building all of systemd/trixie or systemd/sid; very hard to do on bookworm as lots of packages are missing / unavailable (resulted in xsltproc errors when we tried it)
    * manual repackaging of sytemd-ukify/sid only installed some parts; some hooks were missing and part of the base system
    * dist-upgrade to trixie might be possible
    * UKIs aren't required as systemd-book/bookworm has a hook to generate and support multiple Type#1 images
* systemd-boot has Boot counting, i.e., if a boot fails several times, it'll try the next entry
* -> final setup:
    * `/boot` is part of root and contains all vmlinuz, config, and initramfs files
    * `/boot/efi` is the FAT32 ESP and contains systemd-boot and all UKIs
    * UKIs aren't supported on bookworm (systemd-ukify doesn't exist)
    * systemd-boot on bookworm has a BLS hook

## WARNING

* DO NOT USE `dpkg-reconfigure linux-image-XXX` as that will for some reason get rid of the ZFS kernel module
    * if that happened, use `dpkg-reconfigure zfs-dkms` followed by `kernel-install add XXX /boot/initramfs-XXX`

### Switch from GRUB2 to systemd-boot with BLS

First, perform a system backup.
Connect the root SSDs to a separate system.
```
# stop mdadm
mdadm --stop --scan
# backup disk for safety purposes
dd if=/dev/sdX of=readynas-disk1 bs=4M
dd if=/dev/sdZ of=readynas-disk2 bs=4M
```

Reconnect the SSDs to the NAS and turn it back on.
**In the grub boot menu, add the `efi=runtime` kernel option (required for efivars).**
With the NAS running again:
```
# uninstall grub
apt purge grub-common
rm -r /boot/grub /boot/efi/EFI
# may not be needed but some people report a kernel update bringing in grub again
apt-mark hold grub-common grub2-common grub-efi-amd64 grub-efi-amd64-bin grub-efi-amd64-signed

# install systemd-boot
apt install systemd-boot
# relax esp checks is required as we are using a RAID1 boot partition
# which bootctl doesn't allow by default; but it's fine in this setup
SYSTEMD_RELAX_ESP_CHECKS=1 bootctl install
# add `SYSTEMD_RELAX_ESP_CHECKS=1` in front of `bootctl`
vim /etc/kernel/postinst.d/zz-systemd-boot
vim /etc/kernel/postrm.d/zz-systemd-boot
vim /etc/initramfs/post-update.d/systemd-boot
# content: `systemd.gpt_auto=no console=ttyS0,115200n8 efi=runtime quiet root=UUID=<md1-uuid>`
# current cmdline can be gotten via `cat /proc/cmdline`
vim /etc/kernel/cmdline

# add zfs kernel module to boot
apt install zfs-initramfs

kernel-install add $(uname -r) /boot/vmlinuz-$(uname -r)

# delete all old BootXXXX (debian0 and debian1 and Linux Boot Manager)
efibootmgr -b X -B
# add new EFI entries for systemd-boot for both boot-raid devices
efibootmgr -c -d /dev/sdX -L debian0 -l '\EFI\SYSTEMD\SYSTEMD-BOOTX64.EFI'
efibootmgr -c -d /dev/sdZ -L debian1 -l '\EFI\SYSTEMD\SYSTEMD-BOOTX64.EFI'
# set order to debian0,debian1,EFI-Shell
efibootmgr -o 0,2,1

apt autoremove

# confirm zfs is in the initramfs
lsinitramfs /boot/efi/$(cat /etc/machine-id)/$(uname -r)/initrd.img-$(uname -r) | grep zfs
```

Reboot to ensure switching from grub to systemd-boot worked.
Afterwards, still on the NAS:
```
# fail and remove the root-partition of one of the two drives from the raid array
# keep the /boot/efi raid array intact
mdadm /dev/md1 --fail /dev/sdXY
mdadm /dev/md1 --remove /dev/sdXY
mdadm --zero-superblock /dev/sdXY

# change partition type of Partition 2 to Solaris /usr & Apple ZFS
fdisk /dev/sdX

# get the sdXY-id from `ls -al /dev/disk/by-id`
zpool create -f -O mountpoint=none -o ashift=12 -o autotrim=on -O acltype=posixacl -O xattr=sa -O dnodesize=auto -O compression=lz4 -O relatime=on -O canmount=off -O mountpoint=/ -R /mnt rootpool <sdX-id>-part2
zfs create -o canmount=off -o mountpoint=none rootpool/ROOT
zfs create -o canmount=noauto -o mountpoint=/ rootpool/ROOT/debian
zpool set cachefile= rootpool
# check that both `tank` and `rootpool` exist in `zpool.cache`
zdb -U /etc/zfs/zpool.cache

# edit kernel options `root=UUID=X` to `root=ZFS=rootpool/ROOT/debian`
vim /mnt/etc/kernel/cmdline

update-initramfs -c -k $(uname -r)
kernel-install add $(uname -r) /boot/vmlinuz-$(uname -r)

# verify that the new `root=ZFS=...` option is set
vim /mnt/boot/efi/loader/entries/<machine-id>-<uname-r>.conf

# verify that the sizes match
ls -l /etc/zfs/zpool.cache
lsinitramfs -l /boot/efi/$(cat /etc/machine-id)/$(uname -r)/initrd.img-$(uname -r) | grep zpool.cache
```


Poweroff NAS and connect root SSDs to a separate system.
```
mdadm --stop --scan
# assemble single-disk raid array from the non-ZFS root disk
mdadm --assemble /dev/md1 /dev/sdZ2

mkdir -p /mnt2
mount /dev/md1 /mnt

zpool import -fR /mnt2 rootpool
zfs mount rootpool/ROOT/debian

rsync -av --info=progress2 /mnt/ /mnt2/

zpool export rootpool
umount /mnt
mdadm --stop --scan
sync
```

Connect the new ZFS drive to the NAS and boot from it.
When the system is up and running, connect the old non-ZFS root drive.
(During boot an error regarding `systemd-remount-fs` will be shown. This is fine and fixed below.)
```
# check if root is mounted via zfs
mount | grep rootpool

# delete `/` mount rule -> get rid of systemd-remount-fs error
vim /etc/fstab

# check boot-raid is created correctly
lsblk

# if needed, add boot-partition back into raid array
#mdadm /dev/md127 -a /dev/sdZ1

# disable root raid array from old non-ZFS disk
mdadm --stop /dev/md1
mdadm --zero-superblock /dev/sdZ2

# add old non-ZFS root partition to zpool
# get the zfs-disk-id from `zpool status`
# get the sdZ2-id from `ls -al /dev/disk/by-id`
zpool attach -f rootpool <zfs-disk-id> <old-non-ZFS-id>-part2
```

