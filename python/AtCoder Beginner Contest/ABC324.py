def A():
    n = int(input())
    list_a = list(map(int, input().split()))
    value = list_a[0]

    is_same = True
    for a in list_a:
        if a != value:
            is_same = False
            break
    if is_same:
        print("Yes")
    else:
        print("No")
    return


def B():
    n = int(input())
    while True:  # 3のループ
        q, mod = divmod(n, 3)
        if mod == 0:
            n = q
        else:  # nの更新はしない
            break
    while True:  # 2のループ
        q, mod = divmod(n, 2)
        if mod == 0:
            n = q
        else:  # nの更新はしない
            break
    if n == 1:
        print("Yes")
    else:
        print("No")
    return


def _is_same_str(s1, s2):
    if s1 == s2:
        return True
    else:
        return False


def C():
    n, t_recieve = input().split()
    n = int(n)
    list_s = []
    for _ in range(n):
        list_s.append(input())

    len_t_recieve = len(t_recieve)
    list_true = []
    for j, s in enumerate(list_s):
        len_s = len(s)
        if len_s == len_t_recieve:
            for i in range(len_t_recieve):
                new_s = s[:i] + s[i + 1 :]
                new_t = t_recieve[:i] + t_recieve[i + 1 :]
                if _is_same_str(new_s, new_t):
                    list_true.append(j + 1)
                    break
        elif len_s < len_t_recieve:
            for i in range(len_t_recieve):
                new_t = t_recieve[:i] + t_recieve[i + 1 :]
                # print(f"{t_recieve[:i]=}")
                # print(f"{t_recieve[i+1:]=}")
                if _is_same_str(s, new_t):
                    list_true.append(j + 1)
                    break
        elif len_s > len_t_recieve:
            for i in range(len_s):
                new_s = s[:i] + s[i + 1 :]
                if _is_same_str(new_s, t_recieve):
                    list_true.append(j + 1)
                    break
    num_true = len(list_true)
    str_out = ""
    for value in list_true:
        str_out += f"{value} "
    print(num_true)
    print(str_out)
    return


if __name__ == "__main__":
    C()
