from pulp import *


n = int(input())

q = list(map(int, input().split()))

a = list(map(int, input().split()))

b = list(map(int, input().split()))


mokuteki = LpProblem(sense=LpMaximize)

na = LpVariable("na", lowBound=0, cat=LpInteger)

nb = LpVariable("nb", lowBound=0, cat=LpInteger)
mokuteki += na + nb

# val_a = [LpVariable("a%d" % i, cat=LpInteger) for i in range(n)]
# val_b = [LpVariable("b%d" % i, cat=LpInteger) for i in range(n)]
# val_q = [LpVariable("q%d" % i, cat=LpInteger) for i in range(n)]

for i in range(n):
    mokuteki += na * a[i] + nb * b[i] <= q[i]

mokuteki.solve(PULP_CBC_CMD(msg=False))

print(int(value(na) + value(nb)))
