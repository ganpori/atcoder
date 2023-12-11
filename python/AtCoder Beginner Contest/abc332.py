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


def d():
    h, w = map(int, input().split())

    a = [input().split() for _ in range(h)]
    b = [input().split() for _ in range(h)]
    print(a)
    print(b)

    set_a_row = set()
    set_b_row = set()
    set_a_col = set()
    set_b_col = set()

    for i in range(h):
        set_a_row.add(a[i])
        set_b_row.add(set(b[i]))

    print(set_a_row)
    print(set_b_row)
    return


d()
