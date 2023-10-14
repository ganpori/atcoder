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


if __name__ == "__main__":
    B()
