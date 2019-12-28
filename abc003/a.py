import sys

if sys.platform == 'ios':
    sys.stdin = open('input_file.txt')

# +++++

n = int(input().strip())
print((n+1)*10000//2)
