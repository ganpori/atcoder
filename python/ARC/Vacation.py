import numpy as np

n = int(input())
move = []
for i in range(n):
    move.append(list(map(int, input().split())))

dp = [[0] * 3 for _ in range(n)]


dp[0] = move[0]
for i in range(1, n):
    for j in range(3):  # 一個前の行動
        for k in range(3):  # 今回の行動
            if j == k:
                continue
            happiness = move[i][k]
            dp[i][k] = max(dp[i - 1][j] + happiness, dp[i][k])
print(max(dp[-1]))
# print(dp)
# https://qiita.com/drken/items/dc53c683d6de8aeacf5a
# https://qiita.com/drken/items/a5e6fe22863b7992efdb
# https://www.momoyama-usagi.com/entry/info-algo-dp
