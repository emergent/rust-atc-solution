from itertools import permutations
n = int(input())
ps = ''.join(input().split())
qs = ''.join(input().split())
perm = sorted(
    list(map(''.join, (permutations(map(str, list(range(1, n+1))))))))
pi, qi = 0, 0
#print(ps, qs)
# print(len(perm))
for i in range(len(perm)):
    # print(perm[i])
    if perm[i] == ps:
        # print('pi={}'.format(i))
        pi = i+1
    if perm[i] == qs:
        # print('qi={}'.format(i))
        qi = i+1

print(abs(pi-qi))
