#include<stdio.h>
#include<poll.h>
#include<unistd.h>
#include<fcntl.h>

int main() {
  int fd = open("/proc/meminfo", O_RDONLY);
  int buf[100];
  read(fd, buf, 100);
  buf[99] = 0;
  puts(buf);
  struct pollfd fds;
  fds.fd = fd;
  fds.events = POLLPRI;
  while (1) {
    poll(&fds, 1, -1);
    puts("poll");
    read(fd, buf, 100);
    buf[99] = 0;
    puts(buf);
  }
}
