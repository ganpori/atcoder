def A():
    s = input()

    is_zero_even_all = True

    for i, value in enumerate(s):
        if (i + 1) % 2 == 0:
            if value != "0":
                is_zero_even_all = False

    if is_zero_even_all:
        print("Yes")
    else:
        print("No")
    return


if __name__ == "__main__":
    A()
