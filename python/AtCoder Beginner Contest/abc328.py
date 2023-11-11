def b():
    n = int(input())
    list_day = list(map(int, input().split()))

    count = 0
    for month in range(1, n + 1):
        month_str = str(month)
        i = month_str[0]
        for day in range(1, list_day[month - 1] + 1):
            day_str = str(day)
            num_str = month_str + day_str
            if num_str.count(i) == len(num_str):
                count += 1
    print(count)
    return


def c():
    n, q = map(int, input().split())
    s = input()
    list_lr = []
    for _ in range(q):
        a = list(map(int, input().split()))
        list_lr.append(a)

    # 事前に変換
    list_num = [0] * n
    count = 0
    s_old = s[0]
    for i, s_new in enumerate(s[1:]):
        if s_new == s_old:
            count += 1
        list_num[i + 1] = count
        s_old = s_new
    # print(list_num)
    for lr in list_lr:
        # print(lr)
        if lr[0] == lr[1]:
            print(0)
        else:
            value = list_num[lr[1] - 1] - list_num[lr[0] - 1]
            print(value)
    # target_str = s[lr[0] - 1 : lr[1]]
    # print(target_str)
