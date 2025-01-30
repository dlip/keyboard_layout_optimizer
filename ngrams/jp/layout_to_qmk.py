
qwerty_map = {
'カ': 'JK_KA',
'キ': 'JK_KI',
'ク': 'JK_KU',
'ケ': 'JK_KE',
'コ': 'JK_KO',
'サ': 'JK_SA',
'シ': 'JK_SHI',
'ス': 'JK_SU',
'セ': 'JK_SE',
'ソ': 'JK_SO',
'タ': 'JK_TA',
'チ': 'JK_CHI',
'ツ': 'JK_TSU',
'テ': 'JK_TE',
'ト': 'JK_TO',
'ハ': 'JK_HA',
'ヒ': 'JK_HI',
'フ': 'JK_FU',
'ヘ': 'JK_HE',
'ホ': 'JK_HO',
'ヤ': 'JK_YA',
'ユ': 'JK_YU',
'ヨ': 'JK_YO',
'ア': 'JK_A',
'イ': 'JK_I',
'ウ': 'JK_U',
'エ': 'JK_E',
'オ': 'JK_O',
'ラ': 'JK_RA',
'リ': 'JK_RI',
'ル': 'JK_RU',
'レ': 'JK_RE',
'ロ': 'JK_RO',
'マ': 'JK_MA',
'ミ': 'JK_MI',
'ム': 'JK_MU',
'メ': 'JK_ME',
'モ': 'JK_MO',
'ナ': 'JK_NA',
'ニ': 'JK_NI',
'ヌ': 'JK_NU',
'ネ': 'JK_NE',
'ノ': 'JK_NO',
'ワ': 'JK_WA',
'ヲ': 'JK_WO',
'ン': 'JK_N',
'ー': 'JK_SEN',
'。': 'JK_DOT',
'、': 'JK_COM',
'「': 'JK_LQU',
'」': 'JK_RQU',
'・': 'JK_SLS',
'゛': 'JK_TEN',
'゜': 'JK_MAR',
}

def qwerty_remap(string):
    res = ""
    for c in string:
        if c == '\n':
            continue
        if c in qwerty_map:
            res += qwerty_map[c] + ','
        else:    
            res += c
    return res

with open('layout.txt', 'r') as f:
    for line in f:
        print(qwerty_remap(line))
