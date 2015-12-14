class Node:
    def __init__(self, left, right, key, value):
        self.left = left
        self.right = right
        self.key = key
        self.value = value

    def __traverse__(self, currcode, code):
        if self.value != None:
            code[self.value] = currcode
        else:
            if self.left != None:
                self.left.__traverse__(currcode+"0", code)
            if self.right != None:
                self.right.__traverse__(currcode+"1", code)

    def __str__(self):
        return self.___str___(0)

    def ___str___(self, i):
        ind = "\t"*i
        return "%s%s:%s\n%s%s\n%s%s" % (ind, self.value, self.key, ind, self.left.___str___(i+1) if self.left else "\t\tNone", ind, self.right.___str___(i+1) if self.right else "\t\tNone")

    def traverse(self):
        code = {}
        self.__traverse__("", code)
        return code

f = open("/usr/share/wordlists/rockyou.txt", encoding="latin-1")

chars = {}
for l in f.readlines():
    for c in l:
        if c in chars:
            chars[c] += 1
        else:
            chars[c] = 1

roots = []
for k in chars:
    roots.append(Node(None, None, chars[k], k))

def smallest(roots):
    node = roots[0]
    for i in range(1, len(roots)):
        if roots[i].key < node.key:
            node = roots[i]
    roots.remove(node)
    return node

while len(roots) > 1:
    node = smallest(roots)
    node2 = smallest(roots)
    roots.append(Node(node2, node, node.key+node2.key, None))

tree = roots[0]
code = tree.traverse()

for k in code:
    code[k] = len(code[k])

import os,json
s = os.path.basename(f.name)
s = s[:-(len(s.split(".")[-1])+1)] + ".json"
j = json.dump(code, open(s, "w"))

