import sys

if sys.platform =='ios':
	sys.stdin=open('input_file.txt')
	
#+++++
import itertools
n=int(input())
a={}
for _ in range(n):
	s=input().strip()
	t=a.get(s[0], 0)
	a[s[0]]=t+1

ans=0
for x,y,z in itertools.combinations(list('MARCH'), 3):
	ans+=a.get(x, 0)*a.get(y, 0)*a.get(z, 0)
	
print(ans)
