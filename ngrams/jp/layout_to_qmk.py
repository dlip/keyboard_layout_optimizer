
qwerty_map = {
'カ': 'JA_KA',
'キ': 'JA_KI',
'ク': 'JA_KU',
'ケ': 'JA_KE',
'コ': 'JA_KO',
'サ': 'JA_SA',
'シ': 'JA_SHI',
'ス': 'JA_SU',
'セ': 'JA_SE',
'ソ': 'JA_SO',
'タ': 'JA_TA',
'チ': 'JA_CHI',
'ツ': 'JA_TSU',
'テ': 'JA_TE',
'ト': 'JA_TO',
'ハ': 'JA_HA',
'ヒ': 'JA_HI',
'フ': 'JA_FU',
'ヘ': 'JA_HE',
'ホ': 'JA_HO',
'ヤ': 'JA_YA',
'ユ': 'JA_YU',
'ヨ': 'JA_YO',
'ア': 'JA_A',
'イ': 'JA_I',
'ウ': 'JA_U',
'エ': 'JA_E',
'オ': 'JA_O',
'ラ': 'JA_RA',
'リ': 'JA_RI',
'ル': 'JA_RU',
'レ': 'JA_RE',
'ロ': 'JA_RO',
'マ': 'JA_MA',
'ミ': 'JA_MI',
'ム': 'JA_MU',
'メ': 'JA_ME',
'モ': 'JA_MO',
'ナ': 'JA_NA',
'ニ': 'JA_NI',
'ヌ': 'JA_NU',
'ネ': 'JA_NE',
'ノ': 'JA_NO',
'ワ': 'JA_WA',
'ヲ': 'JA_WO',
'ン': 'JA_N',
'ー': 'JA_SEN',
'。': 'JA_DOT',
'、': 'JA_COM',
'「': 'JA_LQU',
'」': 'JA_RQU',
'・': 'JA_SLS',
'゛': 'JA_TEN',
'゜': 'JA_MAR',
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
