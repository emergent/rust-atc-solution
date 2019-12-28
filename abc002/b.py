import sys

if sys.platform == 'ios':
    sys.stdin = open('input_file.txt')

# +++++
s = input().strip()
s2 = ''.join(filter(lambda c: c not in ['a', 'i', 'u', 'e', 'o'], s))
print(s2)
