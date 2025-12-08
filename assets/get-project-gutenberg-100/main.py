#!/usr/bin/env python3

import requests
from datetime import date
from pathlib import Path
from bs4 import BeautifulSoup

# Base content dir
CONTENT = Path("content")
CONTENT.mkdir(exist_ok=True)

# Top dir
TODAY = CONTENT / f"{date.today().isoformat()}_list"
TODAY.mkdir(parents=True, exist_ok=True)


def get_top():
    # top is the name given to the list of top 100 books on project gutenberg.
    r = requests.get('https://www.gutenberg.org/browse/scores/top')
    top_html = TODAY / "top.html"
    top_html.write_text(r.text, encoding="utf-8")


def parse_top() -> list[tuple]:
    """
    returns key pairs from top_html soup.li.name (should be Frankenstein...) and then soup.li.href
    <li><a href="/ebooks/84">Frankenstein; Or, The Modern Prometheus by Mary Wollstonecraft Shelley (5153)</a></li>
    https://www.gutenberg.org/ebooks/84.txt.utf-8
    """
    top_path = TODAY / "top.html"
    top_html = top_path.read_text()
    soup = BeautifulSoup(top_html, 'html.parser')

    results = []
    # hacky solution just select the first 100 listed ol li a
    for a in soup.select("ol li a")[:100]:
        title = a.get_text(strip=True)
        href = a["href"]

        results.append((title, href)) # return list[tuple]

    return results

def build_download_links(parsed_list: list[tuple]) -> list[tuple]:
    """
        example download link
        https://www.gutenberg.org/ebooks/84.txt.utf-8

        Takes list[(title, href)] and returns list[(title, href, download_url)]
    """
    base = "https://www.gutenberg.org"
    suffix = ".txt.utf-8"

    results = []
    for title, href in parsed_list:
        download_url = base + href + suffix
        results.append((title, href, download_url))

    return results

def download_texts(book_list: list[tuple]):
    """
        uses list of urls to download all texts and place them in their respective dir
    """
    for title, href, url in book_list:
        try:
            r = requests.get(url)
            r.raise_for_status()
        except Exception as e:
            print(f"[!] Failed to fetch {url}: {e}")
            continue

        safe_name = title.replace("/", "-")  # protect filesystem
        book_text = TODAY / f"{safe_name}.txt"
        book_text.write_text(r.text, encoding="utf-8")


if __name__ == "__main__":
    get_top()
    book_list_tuple = parse_top()
    book_list = build_download_links(book_list_tuple)
