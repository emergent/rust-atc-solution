n = int(input().strip())

zoros = [int(str(i)*3) for i in range(1,10)]
#print(zoros)

def minzoro(x):
    return min(filter(lambda i: i >= x, zoros))

print(minzoro(n))
