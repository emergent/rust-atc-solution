import sys

if sys.platform =='ios':
	sys.stdin=open('input_file.txt')
	
#+++++
n,m=list(map(int, input().split()))
aa={i:[] for i in range(1,n+1)}
for _ in range(m):
	a,b = list(map(int, input().split()))
	aa[a].append(b)
	aa[b].append(a)
for a in aa[1]:
	if n in aa[a]:
		print("POSSIBLE")
		break
else:
	print("IMPOSSIBLE")
