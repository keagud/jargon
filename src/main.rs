mod gen_entries;
use gen_entries::ENTRIES as jargon_entries;

use colored::Colorize;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use textwrap::fill;

type Entry = [&'static str; 3];

fn match_entry(query: &str, threshold: i64) -> Option<Entry> {
    let matcher = SkimMatcherV2::default();

    let entry_iter = IntoIterator::into_iter(jargon_entries);

    let search_result = entry_iter.max_by_key(|e| matcher.fuzzy_match(query, e[0]))?;

    let score = matcher.fuzzy_match(query, search_result[0])?;

    if threshold == 0 || score < threshold {
        Some(search_result)
    } else {
        None
    }
}

fn format_entry(e: Entry, width: usize) -> String {
    fill(
        match e {
            [title, "", content] => format!("{}\n{}", title.bold().blue(), content),
            [title, pos, content] => {
                format!("{} {}\n{}", title.bold().blue(), pos.italic().magenta(), content)
            }
        }
        .as_str(),
        width,
    )
}

fn main() {
    if let Some(s) = match_entry("zorkmid", 0) {
        println!("{}", format_entry(s, 80));
    } else {
        println!("That didn't work out")
    };
}
