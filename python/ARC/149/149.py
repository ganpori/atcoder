import numpy as np

def A():
    list_int_input = list(map(int,input().split()))
    n = list_int_input[0]
    m = list_int_input[1]

    max_x = -1
    max_digit = n

    # max_value = 10**n
    # factor = max_value/m
    # int_factor = int(factor)
    max_digit = int(n - np.log10(m))
    
    for element_x in range(1,10):
        for digit in range(1,max_digit+1 +):
            candidate_x = int(digit*str(element_x))
            # print(candidate_x)
            if (max_x < candidate_x) and (candidate_x % m == 0):
                max_x = candidate_x
    print(max_x)
    return

if __name__ == "__main__":
    A()