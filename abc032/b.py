s = input().strip()
k = int(input().strip())
a = []
for i in range(len(s)-k+1):
    a.append(s[i:i+k])
print(len(set(a)))
