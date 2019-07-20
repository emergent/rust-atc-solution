from itertools import combinations
from math import sqrt

n, d = list(map(int, input().split()))
xs = [list(map(int, input().split())) for _ in range(n)]
# print(xs)

count = 0
for c in combinations(xs, 2):
    a = c[0]
    b = c[1]
    y = sqrt(sum([(b[i] - a[i]) ** 2 for i in range(len(a))]))
    if y.is_integer():
        count += 1

print(count)
