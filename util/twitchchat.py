#!/usr/bin/env python3
import requests, re, json

video = "";

ts = 0;
res = requests.get("https://rechat.twitch.tv/rechat-messages", params={"video_id": video, "start": 0})
s = res.json()["errors"][0]["detail"]
groups = re.match(r"0 is not between (\d+) and (\d+)", s).groups()
ts = int(groups[0])
end = int(groups[1])
print("Video id: " + str(video))
print("Fetching messages from " + str(ts) + " to " + str(end) + ".")
for ts in range (ts, end, 30):
    res = requests.get("https://rechat.twitch.tv/rechat-messages", params={"video_id": video, "start": ts})
    print(json.dumps(res.json()))
