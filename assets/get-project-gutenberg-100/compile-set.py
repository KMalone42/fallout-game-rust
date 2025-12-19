#!/usr/bin/env python3

import string
from pathlib import Path

BASE = Path('.')
CONTENT = BASE / "content" / "2025-12-07_list"

# Ignore these characters
# !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~0123456789 
BAD_CHARS = string.punctuation + string.digits + "“”‘’"
table = str.maketrans("", "", BAD_CHARS)

token_set = set()
for file_path in CONTENT.glob("*.txt"):

    text = file_path.read_text(encoding="utf-8")

    text = (
        text.replace("—", " ")
        .replace("–", " ")
        .replace("-", " ")  # non-breaking hyphen, just in case
    )

    tokens = set(
        word.translate(table).lower()
        for word in text.split()
        if word.translate(table)
    )

    token_set |= tokens

token_path = BASE / 'tokens.txt'
token_path.write_text("\n".join(sorted(token_set)), encoding="utf-8")

