def a():
    n = input()
    s = input()

    rinsetu = False

    for i, si in enumerate(s[1:]):
        if si == "a" and s[i] == "b":
            rinsetu = True
        elif si == "b" and s[i] == "a":
            rinsetu = True

    if rinsetu:
        print("Yes")
    else:
        print("No")
    return


def b():
    bb = int(input())
    a = 0
    while True:
        a += 1
        result = a**a
        if result == bb:
            print(a)
            break
        elif result > bb:
            print("-1")
            break
    return


def c():
    list_a = []
    for _ in range(9):
        a = list(map(int, input().split()))
        list_a.append(a)
    # print(list_a)
    is_satisfy = True

    # 横
    for a in list_a:
        if sum(a) != 45:
            is_satisfy = False
        if len(set(a)) != 9:
            is_satisfy = False
    # 縦
    for i in range(9):
        tate = []
        for j in range(9):
            tate.append(list_a[j][i])
        if sum(tate) != 45:
            is_satisfy = False
        if len(set(tate)) != 9:
            is_satisfy = False
    # □
    for i in range(3):
        yoko3 = list_a[i * 3 : i * 3 + 3]

        for j in range(3):
            sikaku = []
            sikaku.extend(yoko3[0][j * 3 : j * 3 + 3])
            sikaku.extend(yoko3[1][j * 3 : j * 3 + 3])
            sikaku.extend(yoko3[2][j * 3 : j * 3 + 3])
            if sum(sikaku) != 45:
                is_satisfy = False
            if len(set(sikaku)) != 9:
                is_satisfy = False
    if is_satisfy:
        print("Yes")
    else:
        print("No")
    return


def d():
    import sys

    sys.setrecursionlimit(10**7)
    n, m = map(int, input().split())
    # list_a = list(map(int, input().split()))
    # list_b = list(map(int, input().split()))
    # print(list_a)
    # print(list_b)

    list_edges = [list(map(int, input().split())) for _ in range(2)]
    adjacency_list = [[] for _ in range(n)]
    for a, b in zip(list_edges[0], list_edges[1]):
        # if a == b:
        #     print("No")
        #     exit()
        adjacency_list[b - 1].append(a - 1)
        adjacency_list[a - 1].append(b - 1)

    # print(adjacency_list)

    list_visited = [False] * n
    node_value = [0] * n
    for i in range(n):
        # print(f"start_pos={i}")
        dfs(
            pos=i,
            adjancy_list=adjacency_list,
            list_visited=list_visited,
            node_value=node_value,
        )

    print("Yes")
    return


def dfs(pos, adjancy_list, list_visited, node_value):
    list_visited[pos] = True
    for node_from_pos in adjancy_list[pos]:
        if list_visited[node_from_pos] == False:
            # print(f"{pos=},{node_from_pos=}")
            node_value[node_from_pos] = 1 - node_value[pos]
            dfs(
                pos=node_from_pos,
                adjancy_list=adjancy_list,
                list_visited=list_visited,
                node_value=node_value,
            )
        elif list_visited[node_from_pos] == True and (
            node_value[pos] == node_value[node_from_pos]
        ):
            print("No")
            exit()
        elif list_visited[node_from_pos] == True and (
            node_value[pos] != node_value[node_from_pos]
        ):
            pass
    return


d()
