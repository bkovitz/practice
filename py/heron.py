# Heron's Method of finding a square root

def heron(x):
    result = x * 0.25
    for _ in range(17):
        result = 0.5 * (result + x / result)
        print(result)
    return int(result)

x = heron(2 ** 31 - 1)
