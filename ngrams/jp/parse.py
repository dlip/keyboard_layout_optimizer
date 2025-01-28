import re
import requests

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
            print(token)
        break
