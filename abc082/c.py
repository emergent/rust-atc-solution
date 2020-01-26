import sys

if sys.platform == 'ios':
    sys.stdin = open('input_file.txt')

# +++++
n = int(input())
aa = list(map(int, input().strip().split()))
am = {}
for a in aa:
    t = am.get(a, 0)
    am[a] = t+1

ans = 0
for ak, av in am.items():
    if ak <= av:
        ans += av-ak
    else:
        ans += av
print(ans)
