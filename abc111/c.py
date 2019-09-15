n = int(input())
vs = list(map(int, input().split()))
es = {}
os = {}
for i, v in enumerate(vs):
    if i % 2 == 0:
        v0 = es.get(v, 0)
        es[v] = v0 + 1
    else:
        v0 = os.get(v, 0)
        os[v] = v0 + 1

es_max = max(es, key=es.get)
es_cnt = sum(es.values())
es_ch_len = es_cnt - es[es_max]
del es[es_max]

os_max = max(os, key=os.get)
os_cnt = sum(os.values())
os_ch_len = os_cnt - os[os_max]
del os[os_max]

if os_max == es_max:
    if es == {}:
        es_len = 0
    else:
        es_snd = max(es, key=es.get)
        es_len = es[es_snd]
    if os == {}:
        os_len = 0
    else:
        os_snd = max(os, key=os.get)
        os_len = os[os_snd]
    if es_len > os_len:
        es_ch_len = es_cnt - es_len
    else:
        os_ch_len = os_cnt - os_len

print(os_ch_len+es_ch_len)
