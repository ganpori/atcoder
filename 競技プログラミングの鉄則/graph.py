def a61():
    n, m = map(int, input().split())

    adjacency_list = [[] for _ in range(n)]
    for _ in range(m):
        a, b = map(int, input().split())

        adjacency_list[a - 1].append(b - 1)
        adjacency_list[b - 1].append(a - 1)

    for i in range(n):
        node_str = "{"
        for node in adjacency_list[i]:
            node_str += f"{node+1}, "
        if node_str[-1] == " ":
            node_str = node_str[:-2]
        node_str += "}"
        print(f"{i+1}: {node_str}")
    return


def dfs(pos, adjancy_list, list_visited):
    list_visited[pos] = True
    for node_from_pos in adjancy_list[pos]:
        if list_visited[node_from_pos] == False:
            dfs(pos=node_from_pos, adjancy_list=adjancy_list, list_visited=list_visited)
    return


def a62():
    n, m = map(int, input().split())
    list_edges = [list(map(int, input().split())) for _ in range(m)]
    list_visited = [False] * n
    adjacency_list = [[] for _ in range(n)]
    for a, b in list_edges:
        adjacency_list[b - 1].append(a - 1)
        adjacency_list[a - 1].append(b - 1)

    dfs(pos=0, adjancy_list=adjacency_list, list_visited=list_visited)

    is_connected = all(list_visited)
    if is_connected:
        print("The graph is connected.")
    else:
        print("The graph is not connected.")

    return


if __name__ == "__main__":
    a62()
