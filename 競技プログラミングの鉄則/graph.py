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


if __name__ == "__main__":
    a61()
