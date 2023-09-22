a, k = list(map(int, input().split()))
n = len(str(a))

ans = float("inf")
for i in range(n):  # i桁目まではaと同じ値にする
    for q in range(10):  # i+1桁目をいい感じのやつにする。初めて異なる数を使う場所。何でここが特別かよくわからん、
        for r in range(10):  # 繰り返す数字。
            num = n[:i] + str(q) + str(r) * (n - i - 1)
            if 1 <= len(set(str(num))) <= k:
                ans = min(ans, abs(a - int(num)))
print(ans)
