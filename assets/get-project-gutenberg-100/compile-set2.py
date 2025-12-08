#!/usr/bin/env python3

import re
from pathlib import Path

BASE = Path('.')
CONTENT = BASE / "content" / "2025-12-07_list"

ONLY_ALPHA = re.compile(r"[^A-Za-z]+")

token_set = set()
for file_path in CONTENT.glob("*.txt"):

    text = file_path.read_text(encoding="utf-8")

    text = (
        text.replace("—", " ")
            .replace("–", " ")
            .replace("-", " ")
    )

    tokens = set(
        cleaned.lower()
        for raw in text.split()
        for cleaned in [ONLY_ALPHA.sub("", raw)]
        if cleaned and 4 <= len(cleaned) <= 12
    )

    token_set |= tokens

token_path = BASE / 'tokens.txt'
token_path.write_text("\n".join(sorted(token_set)), encoding="utf-8")
