import random
n = 2*(10**3)
l_list = list(range(1, 10**3))
ls = [random.choice(l_list) for _ in range(n)]
print(n)
print(' '.join(map(str, ls)))
