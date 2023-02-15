#!/bin/env python

import sys
import json
import os
import pathlib


def format_header_line(title, pos, content):

    return f'{{"{title.lower()}", Entry{{ "{title}", "{pos}", "{content}" }}}}'


def make_entry_body(
    filename: str,
):

    with open(filename, "r") as infile:
        file_data = json.load(infile)

        entry_lines = []

    for entry in file_data:
        entry_title = entry["title"]
        entry_pos = entry["pos"]
        entry_content = entry["content"]

        entry_line = format_header_line(entry_title, entry_pos, entry_content)

        entry_lines.append(entry_line)

    return entry_lines


def main():

    all_lines = ["#include <map>", "#include <string>", '#include "entry.hpp"']

    all_lines.append("std::map<std::string, Entry> entries_index{")

    entry_lines = []
    for filename in sys.argv[1:]:
        entry_lines.extend(make_entry_body(filename))

    all_lines.append(",\n".join(entry_lines))

    all_lines.append("\n};")

    file_content = "\n".join(all_lines)

    with open("entries.cpp", "w") as outfile:
        outfile.write(file_content)

    header_path = pathlib.Path("./entries.hpp").resolve()

    if not header_path.exists():

        header_lines = [
            "#ifndef ENTRIES_HPP",
            "#define ENTRIES_HPP",
            "#include <map>",
            "#include <string>",
            '#include "entry.hpp"',
            "extern std::map<std::string, Entry> entries_index;",
            "#endif",
        ]

        with open(header_path, "w") as header_file:
            header_file.writelines([l + "\n" for l in header_lines])


if __name__ == "__main__":
    main()
