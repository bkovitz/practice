#!/bin/python3

import math
import os
import random
import re
import sys
from collections import defaultdict

#
# Complete the 'contacts' function below.
#
# The function is expected to return an INTEGER_ARRAY.
# The function accepts 2D_STRING_ARRAY queries as parameter.
#

def contacts(queries):
    result = []
    num_matches = [defaultdict(int) for _ in range(22)]
    
    for query in queries:
        cmd, name = query
        if cmd == 'add':
            #print(query, len(name))
            for l in range(1, len(name) + 1):
                num_matches[l][name[:l]] += 1
        else:
            result.append(num_matches[len(name)][name])
    #print(num_matches)
    return result

if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    queries_rows = int(input().strip())

    queries = []

    for _ in range(queries_rows):
        queries.append(input().rstrip().split())

    result = contacts(queries)

    fptr.write('\n'.join(map(str, result)))
    fptr.write('\n')

    fptr.close()
