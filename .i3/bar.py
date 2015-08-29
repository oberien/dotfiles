import subprocess
import sys
import json
from math import log

# from http://www.voidware.com/moon_phase.htm
def moon_phase(y, m, d):
    #calculates the moon phase (0-7), accurate to 1 segment.
    #0 = > new moon.
    #4 => full moon.

    #int c,e;
    #double jd;
    #int b;

    if (m < 3):
        y -= 1
        m += 12

    ++m
    c = int(365.25*y)
    e = int(30.6*m)
    # jd is total days elapsed
    jd = c+e+d-694039.09 
    # divide by the moon cycle (29.53 days)
    jd /= 29.53
    # int(jd) -> b, take integer part of jd
    b = int(jd)
    # subtract integer part to leave fractional part of original jd
    jd -= b
    # scale fraction from 0-8 and round by adding 0.5
    b = int(jd*8 + 0.5)
    # 0 and 8 are the same so turn 8 into 0
    b = b & 7
    return b

def runProcess(exe):    
    p = subprocess.Popen(exe, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    while(True):
      retcode = p.poll() #returns None while subprocess is running
      line = p.stdout.readline();
      yield line
      if(retcode is not None):
        break


def convertKB(i):
    ending = ["k", "M", "G", "T"]
    if (i == 0):
        return "0k"
    e = int(log(i, 1024));
    n = round(i / 1024**e, 1)
    return str(n) + ending[e]

i = 0
s = "full_text"
x = "\U000020E0"
icons = ["\U0001F4BD", "\U0001F500", "\U0001F4F6", "\U0001F5A7",
        {"BAT":"\U0001F50B", "CHR":"\U0000231B", "FULL":"\U0001F50C"},
        "\U0001F4BB",["\U000026A0", "\U000026A1"],
        ["\U0001F55B","\U0001F567","\U0001F550","\U0001F55C","\U0001F551",
            "\U0001F55D","\U0001F552","\U0001F55E","\U0001F553","\U0001F55F",
            "\U0001F554","\U0001F560","\U0001F555","\U0001F561","\U0001F556",
            "\U0001F562","\U0001F557","\U0001F563","\U0001F558","\U0001F564",
            "\U0001F559","\U0001F565","\U0001F55A","\U0001F566"]]
icons2 = [["\U0001F505", "\U0001F506"], "\U0001F40F", "\U0001F4BE",
        ["\U0001F303", "\U0001F306", "\U0001F305", "\U0000263C", "\U0001F307"],
        "\U0001F4C6"]
for line in runProcess('i3status'):
    i += 1
    if (i == 1):
        print('{"version": 1}')

    elif (i == 2):
        print('[')

    else:
        res = []
        start = 2 + (i>3)
        line = str(line)[start:-3]

        obj = json.loads(line)

        brightness = {"name": "brightness"}
        try:
            with open("/sys/class/backlight/acpi_video0/actual_brightness", 'r') as f:
                b = int(f.read().replace("\n", ""))
            brightness[s] = icons2[0][min(b, 99)//50] + " " + str(b) 
            res.append(brightness)
        except FileNotFoundError:
            pass

        m = obj[0][s]
        obj[0][s] = icons[0] + " " + m
        res.append(obj[0])

        m = obj[1][s]
        obj[1][s] = icons[1] + (x if m == "" else " " + m)
        res.append(obj[1])

        m = obj[2][s]
        obj[2][s] = icons[2] + (x if m == "" else " " + m)
        res.append(obj[2])

        m = obj[3][s]
        obj[3][s] = icons[3] + (x if m == "" else " " + m)
        res.append(obj[3])

        m = obj[4][s]
        if (m != "No battery"):
            m = m.split(" ")
            c = m[0]
            m = m[1]
            obj[4][s] = icons[4][c] + " " + m
            res.append(obj[4])

        m = obj[5][s]
        obj[5][s] = icons[5] + " " + m
        res.append(obj[5])

        meminfo = {"name": "meminfo"}
        with (open("/proc/meminfo", 'r')) as f:
            mem = f.read()
        mem = "{\"" + mem.replace("\n", "\",\"").replace(":", "\":\"")[:-2] + "}"
        mem = json.loads(mem)
        total = int(mem["MemTotal"].split(" ")[-2])
        avail = int(mem["MemAvailable"].split(" ")[-2])
        used = total - avail
        meminfo[s]  = icons2[1] + " " + convertKB(used) + "/" + convertKB(total)
        res.append(meminfo)

        m = obj[6][s]
        obj[6][s] = icons[6]["color" in obj[6] and obj[6]["color"] == "#FF0000"] + " " + m
        res.append(obj[6])

        swap = {"name": "swap"}
        total = int(mem["SwapTotal"].split(" ")[-2])
        avail = int(mem["SwapFree"].split(" ")[-2])
        used = total - avail
        swap[s]  = icons2[2] + " " + convertKB(used) + "/" + convertKB(total)
        res.append(swap)

        m = obj[7][s]
        h = int(m[:2])
        hm = h % 12
        t = int(m[3:5]) + 15
        t = t//30 % 2
        m = m.split(" ")
        ymd = m[1].split("/")
        moon = moon_phase(int(ymd[2]), int(ymd[0]), int(ymd[1]))
        sun = icons2[3][h//5]
        obj[7][s] = icons[7][hm*2+t] + " " + m[0] + " " + sun + " " + icons2[4] + " " + m[1] + " " + chr(moon+0x1F311)
        res.append(obj[7])

        print(("," if i > 3 else "") + json.dumps(res))
    sys.stdout.flush()
