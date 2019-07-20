import random
N = 1000
Q = 1000
a = ['A', 'C', 'G', 'T']
s = ''.join([random.choice(a) for _ in range(N)])
l = list(range(1, len(s)))

print('{} {}'.format(N, Q))
print(s)
for _ in range(Q):
    li = random.choice(l)
    ri = random.choice(range(li+1, len(s)+1))
    print('{} {}'.format(li, ri))
