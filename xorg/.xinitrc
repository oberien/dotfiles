autocutsel -fork &
#autocutsel -selection PRIMARY -fork &

xrdb -merge .Xresources

exec i3 -V >> ~/.i3/log/$(date +'%F-%k-%M-%S') 2>&1
