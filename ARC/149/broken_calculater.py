a, border = list(map(int, input().split()))
n = len(str(a))

ans = float("inf")


for i in range(n):  # i桁目まではaと同じ値にする
    for j in range(10):  # i+1桁目をいい感じのやつにする。初めて異なる数を使う場所。何でここが特別かよくわからん、
        for k in range(10):  # 繰り返す数字。
            num = str(a)[:i] + str(j) + str(k) * (n - i - 1)
            num = int(num)
            if 1 <= len(set(str(num))) <= border:
                ans = min(ans, abs(a - int(num)))
print(ans)
