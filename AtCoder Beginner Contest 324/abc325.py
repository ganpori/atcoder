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


if __name__ == "__main__":
    B()
