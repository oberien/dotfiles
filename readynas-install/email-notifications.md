# Email Notifications (msmtp)

### Setup E-Mail client

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
