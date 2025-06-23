# mmult1.py -- A little practice with matrix multiplication

from numbers import Number
#from typing import List


Matrix = list[list[Number]]

def mmult(A: Matrix, B: Matrix) -> Matrix:
    A_rows = len(A)
    A_columns = len(A[0])
    B_rows = len(B)
    B_columns = len(B[0])
    assert A_columns == B_rows
    result = []  # will have A_rows rows and B_columns columns
    for i in range(A_rows):
        result_row = []
        for j in range(B_columns):
            result_row.append(
                sum(A[i][jj] * B[jj][j] for jj in range(B_columns))
            )
        result.append(result_row)
    return result

def mat_to_str(A: Matrix) -> None:
    result = ''
    for row in A:
        result += ' '.join(format(x, '3') for x in row) + '\n'
    return result


A = [[1, 2], [13, 4]]
B = [[1, 3], [2, 4]]
#print(mat_to_str(A))
C = mmult(A, B)
print(mat_to_str(C))
