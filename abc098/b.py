n = int(input())
s = input().strip()
ans = 0
for i in range(len(s)):
    s1 = list(set(s[:i]))
    s2 = list(set(s[i:]))
    count_inner = 0
    for c in s1:
        if c in s2:
            count_inner += 1
    ans = max(ans, count_inner)
print(ans)
