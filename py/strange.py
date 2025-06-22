# Given a number n, does there exist a number x such that x**x + x = n?
#
# Constraint: 0 <= n <= 2**32
import sys

#print(int(sys.argv[1]))

strange_numbers = set([
    1,
    2,
    6,
    30,
    260,
    3130,
    46662,
    823550,
    16777224,
    387420498
])

"""
for i in range(100):
    print(i, i**i + i)
"""

n = int(sys.argv[1])
print(n in strange_numbers)
