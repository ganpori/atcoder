def a():
    x, y = map(int, input().split())
    is_stairs = False
    diff = x - y
    if -2 <= diff <= 3:
        print("Yes")
    else:
        print("No")
    return


def b():
    n = int(input())
    for i in range(n, 920):
        i_str = str(i)
        digit_1 = int(i_str[2])
        digit_10 = int(i_str[1])
        digit_100 = int(i_str[0])
        if digit_10 * digit_100 == digit_1:
            print(i)
            break
    return


if __name__ == "__main__":
    b()
