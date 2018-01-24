#include<stdio.h>
#include<poll.h>
#include<unistd.h>
#include<fcntl.h>

int main() {
  int fd = open("/sys/class/power_supply/BAT0/status", O_RDONLY);
  int buf[100];
  read(fd, buf, 100);
  buf[99] = 0;
  puts(buf);
  struct pollfd fds;
  fds.fd = fd;
  fds.events = POLLPRI;
  lseek(fd, 0, SEEK_SET);
  while (1) {
    poll(&fds, 1, -1);
    puts("poll");
    lseek(fd, 0, SEEK_SET);
    read(fd, buf, 100);
    buf[99] = 0;
    puts(buf);
  }
}
