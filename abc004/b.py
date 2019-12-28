import sys

if sys.platform == 'ios':
    sys.stdin = open('input_file.txt')

# +++++

c = []
for _ in range(4):
    c.append(list(input().split()))

a = [None, None, None, None]
for i in range(4):
    a[3-i] = reversed(c[i])

for i in range(4):
    print(' '.join(a[i]))
