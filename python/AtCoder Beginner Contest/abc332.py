def a():
    n, s, k = map(int, input().split())

    pq = [list(map(int, input().split())) for _ in range(n)]

    price = 0

    for ppqq in pq:
        price += ppqq[0] * ppqq[1]

    if price < s:
        price += k

    print(price)
    return


def b():
    k, g, m = map(int, input().split())

    g_now = 0
    m_now = 0
    for i in range(k):
        if g_now == g:
            g_now = 0
        elif m_now == 0:
            m_now = m
        else:
            if g_now + m_now <= g:
                g_now += m_now
                m_now = 0
            elif g_now + m_now > g:
                m_now = g_now + m_now - g
                g_now = g
    print(f"{g_now} {m_now}")
    return


def c():
    n, m = map(int, input().split())
    s = input()

    count_0 = s.count("0")
    if count_0 == 0:
        num_buy = _calc(s, m)
    elif count_0 == 1:
        index0 = s.find("0")
        num_buy = max(_calc(s[index0:], m), _calc(s[:index0], m))
    elif count_0 >= 2:
        last0 = s.rfind("0")
        num_buy = 0
        first_index = 0
        next_index = s.find("0")
        while next_index <= last0:
            s_target = s[first_index:next_index]
            num_buy = max(num_buy, _calc(s_target, m))
            first_index = next_index
            if next_index == last0:
                break
            next_index = s.find("0", first_index + 1)
        s_target = s[next_index:]
        num_buy = max(num_buy, _calc(s_target, m))

    print(num_buy)
    return


def _calc(s, m):
    count_1 = s.count("1")
    count_2 = s.count("2")
    if m >= count_1:
        num_buy = count_2
    elif m < count_1:
        num_buy = count_2 + (count_1 - m)

    return num_buy


def list2tuple(l):
    t = tuple(tuple(target) for target in l)
    return t


# 交換を行き来可能なノードとみることが可能。
# 盤面の状態そのものをposと置き換えてみる
# distの管理にmapを用いる。mapのkeyにtupleを適用
def d():
    import copy
    from collections import deque

    h, w = map(int, input().split())

    a = [input().split() for _ in range(h)]
    b = [input().split() for _ in range(h)]

    q = deque()
    q.append(a)
    dist = {list2tuple(a): 0}
    while len(q) > 0:
        status = q.popleft()
        tuple_status = list2tuple(status)
        for i in range(h - 1):
            next_status = copy.deepcopy(status)
            next_status[i], next_status[i + 1] = next_status[i + 1], next_status[i]
            tuple_next = list2tuple(next_status)
            if tuple_next in dist.keys():
                continue
            else:
                q.append(next_status)
                dist[tuple_next] = dist[tuple_status] + 1
        for j in range(w - 1):
            next_status = copy.deepcopy(status)
            for i in range(h):
                next_status[i][j], next_status[i][j + 1] = (
                    next_status[i][j + 1],
                    next_status[i][j],
                )
            tuple_next = list2tuple(next_status)
            if tuple_next not in dist:
                q.append(next_status)
                dist[tuple_next] = dist[tuple_status] + 1
    if list2tuple(b) in dist:
        print(dist[list2tuple(b)])
    else:
        print(-1)
    return


d()
