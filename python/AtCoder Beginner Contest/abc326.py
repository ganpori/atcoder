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


def c():
    n, m = map(int, input().split())
    list_a = list(map(int, input().split()))
    list_a.sort()
    max_num = 1
    list_diff = [0] * n
    if len(list_a) == 0:
        print("0")
        return
    if n == 1:
        print("1")
        return
    if max(list_a) - min(list_a) < m:
        print(f"{len(list_a)}")
        return

    for i in range(1, n):
        list_diff[i - 1] = list_a[i] - list_a[i - 1]

    for i in range(len(list_diff)):
        sum_diff = 0
        num_sum = 0
        for diff in list_diff[i:]:
            sum_diff += diff
            num_sum += 1
            if sum_diff >= m:
                max_num = max(num_sum, max_num)
                break

    print(max_num)
    return


if __name__ == "__main__":
    c()
