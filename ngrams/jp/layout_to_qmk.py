
qwerty_map = {
'カ': 'JP_KA',
'キ': 'JP_KI',
'ク': 'JP_KU',
'ケ': 'JP_KE',
'コ': 'JP_KO',
'サ': 'JP_SA',
'シ': 'JP_SHI',
'ス': 'JP_SU',
'セ': 'JP_SE',
'ソ': 'JP_SO',
'タ': 'JP_TA',
'チ': 'JP_CHI',
'ツ': 'JP_TSU',
'テ': 'JP_TE',
'ト': 'JP_TO',
'ハ': 'JP_HA',
'ヒ': 'JP_HI',
'フ': 'JP_FU',
'ヘ': 'JP_HE',
'ホ': 'JP_HO',
'ヤ': 'JP_YA',
'ユ': 'JP_YU',
'ヨ': 'JP_YO',
'ア': 'JP_A',
'イ': 'JP_I',
'ウ': 'JP_U',
'エ': 'JP_E',
'オ': 'JP_O',
'ラ': 'JP_RA',
'リ': 'JP_RI',
'ル': 'JP_RU',
'レ': 'JP_RE',
'ロ': 'JP_RO',
'マ': 'JP_MA',
'ミ': 'JP_MI',
'ム': 'JP_MU',
'メ': 'JP_ME',
'モ': 'JP_MO',
'ナ': 'JP_NA',
'ニ': 'JP_NI',
'ヌ': 'JP_NU',
'ネ': 'JP_NE',
'ノ': 'JP_NO',
'ワ': 'JP_WA',
'ヲ': 'JP_WO',
'ン': 'JP_N',
'ー': 'JP_SEN',
'。': 'JP_DOT',
'、': 'JP_COM',
'「': 'JP_LQU',
'」': 'JP_RQU',
'・': 'JP_SLS',
'゛': 'JP_TEN',
'゜': 'JP_MAR',
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
