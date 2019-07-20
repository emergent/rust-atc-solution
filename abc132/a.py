s = str(input())
d = {}
for c in list(s):
    count = d.get(c, 0)
    d[c] = count + 1

if len(d) == 2 and all(filter(lambda x: x == 2, d.values())):
    print("Yes")
else:
    print("No")
