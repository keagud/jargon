#!/bin/env python

import requests
from bs4 import BeautifulSoup
import json
from concurrent import futures
import yaml


def get_url_info(sub_url: str):

    sub_page = requests.get(sub_url)

    print(f"Getting data on {sub_url}", end="... ")

    try:
        sub_page.raise_for_status()
    except requests.HTTPError:
        print("failed!")
        return sub_url, None


    sub_soup = BeautifulSoup(sub_page.text, features="html.parser")

    entry_header = sub_soup.select_one("dt[id] b")
    entry_content = sub_soup.select("dd")
    entry_pos = sub_soup.select_one(".grammar")

    if entry_header is None or entry_content is None:
        print("failed!")
        return sub_url, None

    title = entry_header.text

    content = {}
    for i, section in enumerate([e.text  for e in entry_content], start=1):
        content["section_" + str(i)] = section
    pos = entry_pos.text if entry_pos is not None else ""

    formatted_dict = {"title": title, "pos": pos, "content": content}
    print("done!")
    return (sub_url, formatted_dict)


def main():
    url = "http://www.catb.org/~esr/jargon/html/go01.html"
    base_url = "http://www.catb.org/~esr/jargon/html"

    page = requests.get(url)

    soup = BeautifulSoup(page.text, features="html.parser")

    failures = []

    all_entries = []

    urls_index = [f"{base_url}/{s['href']}" for s in soup.select("dt>a")]

    with futures.ThreadPoolExecutor() as executor:
        submitted_urls = {executor.submit(get_url_info, u) for u in urls_index}

        for page_future in futures.as_completed(submitted_urls):

            page_url, page_result = page_future.result()

            if page_result is None:
                failures.append(page_url)
                continue

            all_entries.append(page_result)
            

    all_entries.sort(key=lambda x: x['title'])

    with open("jargon.yaml", "w") as outfile:
        yaml.dump(all_entries, outfile, indent=4)

    with open("failed.txt", "w") as failed_file:
        failed_file.writelines([f + "\n" for f in failures])

if __name__ == '__main__':
    main()
