a, border = list(map(int, input().split()))
n = len(str(a))

ans = float("inf")


for i in range(n):
    for j in range(10):
        for k in range(10):
            num = str(a)[:i] + str(j) + str(k) * (n - i - 1)
            num = int(num)
            if 1 <= len(set(str(num))) <= border:
                ans = min(ans, abs(a - int(num)))

print(ans)
