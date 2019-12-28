import fractions
import sys

if sys.platform == 'ios':
    sys.stdin = open('input_file.txt')

# +++++

s = input()
t = input()
c = list('atcoder@')
for i in range(len(s)):
    if not (s[i] == t[i] or (s[i] == '@' and t[i] in c) or (t[i] == '@' and s[i] in c)):
        print('You will lose')
        break
else:
    print('You can win')
