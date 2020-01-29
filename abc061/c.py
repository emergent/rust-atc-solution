import sys

if sys.platform =='ios':
	sys.stdin=open('input_file.txt')
	
#+++++
n,k=list(map(int, input().split()))
aa={}
for _ in range(n):
	a,b = list(map(int, input().split()))
	t = aa.get(a,0)
	aa[a] = t+b

sa = sorted(aa.keys())
for a in sa:
	if k > aa[a]:
		k -= aa[a]
	else:
		print(a)
		break
