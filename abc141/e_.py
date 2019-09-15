import random
import string
N = 5000
a = [random.choice(string.ascii_lowercase) for i in range(N)]
print(N)
print(''.join(a))
