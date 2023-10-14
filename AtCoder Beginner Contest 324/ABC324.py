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


if __name__ == "__main__":
    A()
