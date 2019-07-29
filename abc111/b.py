n = int(input().strip())

def s19(s):
    if s is '9':
        return '1'
    elif s is '1':
        return '9'
    else:
        return s

ns = ''.join(list(map(s19, str(n))))
print(ns)
