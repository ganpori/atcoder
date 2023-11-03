n, w = map(int, input().split())
goods = []
for i in range(n):
    goods.append(list(map(int, input().split())))

dp = [[0] * w for _ in range(n)]


for i in range(n):
    weight, value = goods[i]
    for w_i in range(w):
        limit_weight = w_i + 1
        weight_index_to_compare = max(w_i - weight, 0)
        if limit_weight - weight >= 0:
            dp[i][w_i] = max(value + dp[i - 1][weight_index_to_compare], dp[i - 1][w_i])
        else:
            dp[i][w_i] = dp[i - 1][w_i]

        # print(dp)

print(dp[-1][-1])
# print(dp)
