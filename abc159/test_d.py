import random
l = [1, 2, 3, 4, 5]
N = 200000
print(N)
print(' '.join([str(random.choice(l)) for _ in range(N)]))
