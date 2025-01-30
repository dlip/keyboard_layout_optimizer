import re
import requests
def ngram(n, text):
    tokens = []
    length = len(text)
    if (length < n):
        return tokens

    i = length - n + 1
    chars = list(text)
    while i > 0:
        i -= 1
        tokens.append(''.join(chars[i:i + n]))

    return tokens

def is_all_katakana(string):
    # Match only if the string contains one or more Katakana characters and nothing else
    return bool(re.fullmatch(r"[\u30A0-\u30FF]+", string))

shifted = {
    'ガ':'カ',
    'ギ':'キ',
    'グ':'ク',
    'ゲ':'ケ',
    'ゴ':'コ',
    'ザ':'サ',
    'ジ':'シ',
    'ズ':'ス',
    'ゼ':'セ',
    'ゾ':'ソ',
    'ダ':'タ',
    'ヂ':'チ',
    'ヅ':'ツ',
    'ッ':'ツ',
    'デ':'テ',
    'ド':'ト',
    'バ':'ハ',
    'ビ':'ヒ',
    'ブ':'フ',
    'ベ':'ヘ',
    'ボ':'ホ',
    'パ':'ハ',
    'ピ':'ヒ',
    'プ':'フ',
    'ペ':'ヘ',
    'ポ':'ホ',
    'ャ':'ヤ',
    'ュ':'ユ',
    'ョ':'ヨ',
    'ァ':'ア',
    'ィ':'イ',
    'ゥ':'ウ',
    'ェ':'エ',
    'ォ':'オ'
}

def unshift(string):
    res = ""
    for c in string:
        res += shifted.get(c,c)
    return res

ngrams = [{},{},{}]

with open('core10k.txt', 'r') as f:
    for line in f:
        raw = re.sub(r"\[.*?\]", "", line)
        raw = re.sub(r"\<.*?\>", "", raw)
        raw = raw.replace(" ", "")
        # docker run --rm -p 8000:8000 -t himkt/konoha
        res = requests.post('http://localhost:8000/api/v1/tokenize',json={"tokenizer": "mecab", "text": raw})
        tokens = res.json()['tokens']
        print(raw)
        for token in tokens:
            for x in range(3):
                yomi=token['yomi']
                if yomi is None:
                    if is_all_katakana(token['surface']):
                        yomi=token['surface']
                    else:
                        continue
                yomi = unshift(yomi)
                for n in ngram(x+1,yomi):
                    ngrams[x][n] = ngrams[x].get(n,0) + 1

for x in range(3):
    objs = [{'gram': key, 'count':value} for key, value in ngrams[x].items()]
    objs.sort(key=lambda x: x["count"], reverse=True)

    with open(f"{x+1}-grams.txt", "w") as fp:
        print(f"Writing {fp.name}")
        for o in objs:
            fp.write(f"{o['count']} {o['gram']}\n")
