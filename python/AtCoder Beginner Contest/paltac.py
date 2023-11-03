target_str = "aaaabbcccccccz"
# 　これをa4b2c7z1に圧縮

dict_str_comp = {}
set_char = set(target_str)  # 順序が保障されなかったので、順序保障されたものに変更必要

# 数え挙げて記憶
for target_char in set_char:
    num_target_char = target_str.count(target_char)
    dict_str_comp.update({target_char: num_target_char})
print(dict_str_comp)

# outputの文字列作成
output_str = ""
for key in dict_str_comp:
    output_str += key + str(dict_str_comp[key])
print(output_str)
