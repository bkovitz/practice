import collections
import bisect

table = collections.OrderedDict()
i = 1
big = 2147483647
while True:
    ii = i * i
    if ii > big:
        break
    table[ii] = i
    i += 1

keys = list(table.keys())

ind = bisect.bisect_left(keys, 1000)

print(ind, table[keys[ind - 1]])
