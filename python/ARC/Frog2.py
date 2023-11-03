n, k = map(int, input().split())
h = list(map(int, input().split()))

# n, k = 10, 4
# h = list(map(int, "40 10 20 70 80 10 20 70 80 60".split()))

dp = [float("inf")] * n
dp[0] = 0
dp[1] = abs(h[1] - h[0])
for i in range(2, n):
    for j in range(1, k + 1):
        if i - j >= 0:
            cost_j = abs(h[i] - h[i - j])
            dp[i] = min(dp[i - j] + cost_j, dp[i])
print(dp[-1])
# print(dp)
