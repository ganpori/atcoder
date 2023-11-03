p = int(input())


list_coin = [1]
for i in range(1, 11):
    list_coin.append(i * list_coin[i - 1])
list_coin.pop(0)


num_coin = 0
p_remains = p
list_coin.reverse()
for coin in list_coin:
    num_coin += p_remains // coin
    p_remains = p_remains % coin
print(num_coin)
