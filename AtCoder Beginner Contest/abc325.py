import collections


def A():
    s, t = input().split()
    print(f"{s} san")
    return


def B():
    n = int(input())
    dict_num = {i: 0 for i in range(24)}

    for _ in range(n):
        w, x = map(int, input().split())
        for start_time_i in range(24):
            local_start_time = start_time_i + x
            if local_start_time >= 24:
                local_start_time -= 24
            if 9 <= local_start_time <= 17:
                dict_num[start_time_i] = dict_num[start_time_i] + w

    print(max(dict_num.values()))
    # print(dict_num)
    return


def C():
    h, w = map(int, input().split())
    list_s = []
    for _ in range(h):
        list_s.append(input())
    print(list_s)
    num_sensor = 0

    # 消していって残った#の数を数える
    # 1行目
    s1 = list_s[0]

    # h-1行目

    for i in range(1, h - 1):
        for j in range(w):
            pass
    return


if __name__ == "__main__":
    C()
