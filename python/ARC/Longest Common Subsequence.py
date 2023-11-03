# n = int(input())

# list_x = []
# list_y = []
# for _ in range(n):
#     list_x.append(input())
#     list_y.append(input())


def search_lcs(x, y):
    length = 0
    list_lcs = []
    if len(x) < len(y):  # 必ずxの方が長いようにする
        x, y = y, x
    for len_lcs in range(len(y)):
        for i_y in range(len(y)):
            candidate_lcs = y[i_y : i_y + len_lcs]
            for i_x in range(len(x)):
                if x[i_x : i_x + len_lcs] == candidate_lcs:
                    lcs = candidate_lcs
    return length, lcs


x = "abc"
y = "bc"

search_lcs(x, y)
