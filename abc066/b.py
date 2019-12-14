s = input()
for i in range(2, len(s)+1, 2):
    s2 = s[:-i]
    s2len = len(s2)
    if s2[:s2len//2] == s2[s2len//2:]:
        print(s2len)
        break
