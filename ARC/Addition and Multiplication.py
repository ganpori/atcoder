n = input()
k = input()
# n, k = 10, 10

ans = 1

# 貪欲法
for i in range(int(n)):
    twice = ans * 2
    plus = ans + int(k)
    if twice > plus:
        ans = plus
    else:
        ans = twice
print(ans)
