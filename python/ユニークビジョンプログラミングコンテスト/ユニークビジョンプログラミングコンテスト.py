def A():
    s = input()

    is_zero_even_all = True

    for i, value in enumerate(s):
        if (i + 1) % 2 == 0:
            if value != "0":
                is_zero_even_all = False

    if is_zero_even_all:
        print("Yes")
    else:
        print("No")


def B():
    n = int(input())
    list_s = []
    for _ in range(n):
        list_s.append(input())

    dict_win_number = {}
    for i, s in enumerate(list_s):
        num_win = s.count("o")
        dict_win_number[f"{i+1}"] = num_win
    list_sorted = sorted(dict_win_number.items(), key=lambda x: x[1], reverse=True)
    output = ""
    for l in list_sorted:
        k, v = l
        output = output + f"{k} "
    print(output[:-1])
    return


def C():
    n, m = map(int, input().split())
    list_a = list(map(int, input().split()))
    list_s = []
    for _ in range(n):
        list_s.append(input())
    list_point = []
    list_not_resolve = []
    for i, s_i in enumerate(list_s):
        point = i + 1  # i番目のプレーヤにボーナスiポイント
        target_not_resolve = []
        for j, s_value in enumerate(s_i):
            if s_value == "o":
                point += list_a[j]
            else:
                target_not_resolve.append(list_a[j])
        target_not_resolve.sort(reverse=True)
        list_not_resolve.append(target_not_resolve)
        list_point.append(point)

    max_point = max(list_point)
    # print(list_point)
    # print(max_point)
    # print(list_not_resolve)

    list_num = []
    for i in range(n):
        target_not_resolve = list_not_resolve[i]
        num = 0
        point = list_point[i]
        for not_sesolve in target_not_resolve:
            if point < max_point:
                point += not_sesolve
                num += 1
        list_num.append(num)
        print(num)
    # print(list_num)
    return


def D():
    from collections import defaultdict
    import heapq

    # 優先度付きキューを使うと計算量抑えられる
    # https://kakedashi-engineer.appspot.com/2020/03/18/heapq/
    # https://qiita.com/ell/items/fe52a9eb9499b7060ed6
    #     優先度付きキュー (Priority queue) はデータ型の一つで、具体的には
    # 最小値（最大値）をO(logN)で取り出す
    # 要素をO(logN)で挿入する
    # ことが出来ます。通常のリストだとそれぞれO(N)ですので高速です。

    n = int(input())
    dict_slime = defaultdict(int)  # 初期値0で存在しないkeyも呼び出せる
    for _ in range(n):
        s, c = map(int, input().split())
        dict_slime[s] = c

    size_queue = list(dict_slime.keys())
    heapq.heapify(size_queue)
    # print(dict_slime)
    # ループの判定にNのオーダーかかってたからO(N**2)になって計算遅かった？

    while size_queue:
        size = heapq.heappop(size_queue)
        new_size = size * 2
        new_num = dict_slime[size] // 2

        if new_num == 0:  # もともと１ぴきしかいない
            continue
        else:  # 2匹以上いるとき
            dict_slime[size] = dict_slime[size] % 2  # 0か1
            dict_slime[new_size] += new_num  # defaultdictなら初期値0で存在しないkeyも使える
            if dict_slime[new_size] > 1:  #
                heapq.heappush(size_queue, new_size)

    all_slime = sum(dict_slime.values())
    print(all_slime)
    return


if __name__ == "__main__":
    D()
