# MQTT

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
