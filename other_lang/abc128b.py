n = int(input())
book = []
for i in range(n):
    name, rate = input().split()
    book.append((i+1, name, int(rate)))
b = sorted(book, key=lambda s: s[2], reverse=True)
b2 = sorted(b, key=lambda s: s[1])

for i, _, _ in b2:
    print(i)
