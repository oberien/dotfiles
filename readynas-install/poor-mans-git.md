# Poor Man's git

Modified from <https://git-scm.com/book/en/v2/Git-on-the-Server-Setting-Up-the-Server>:
```sh
zfs create -o encryption=aes-256-gcm -o keyformat=raw -o keylocation=file:///keys/keyfile_zfs -o atime=off -o compression=lz4 -o dedup=on -o mountpoint=/home/git tank/git-home
adduser git
mkdir /home/git/.ssh
chmod 700 /home/git/.ssh
echo "# always prepend 'no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty'" > /home/git/.ssh/authorized_keys
chmod 600 /home/git/.ssh/authorized_keys
chown -R git:git /home/git
echo `which git-shell` >> /etc/shells
chsh git -s $(which git-shell)

# add new empty repository
sudo -iu git
git init --bare foo.git
```

