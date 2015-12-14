import json
code = json.load(open("rockyou.json"))
text = "Password1"

length = 0
for c in text:
    length += code[c]
print(length)
