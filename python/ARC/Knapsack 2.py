n, w = map(int, input().split())
goods = []
for i in range(n):
    goods.append(list(map(int, input().split())))


max_value = 0
max_weight = 0
for i in range(n):
    max_value += goods[i][1]
    max_weight += goods[i][0]


dp = [[float("inf")] * max_value for _ in range(n)]

for value_i in range(max_value):
    weight, value = goods[0]
    if w >= weight and value_i + 1 <= value:
        dp[0][value_i] = goods[0][0]

for i in range(1, n):
    weight, value = goods[i]
    for value_i in range(max_value):
        value_index_to_compare = max(value_i - value, 0)

        if w >= weight + dp[i - 1][value_index_to_compare]:
            dp[i][value_i] = min(
                weight + dp[i - 1][value_index_to_compare], dp[i - 1][value_i]
            )
        else:
            dp[i][value_i] = dp[i - 1][value_i]

        print(dp[i])

# print(dp[-1][-1])
print(dp)
vi = 0
for i in range(n):
    for value_i in range(max_value):
        if dp[i][value_i] != float("inf"):
            vi = max(value_i, vi)
print(f"{vi+1}")
