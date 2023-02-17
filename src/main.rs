mod gen_entries;
use gen_entries::ENTRIES as jargon_entries;

use std::env;

use colored::Colorize;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rand::Rng;
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
                format!(
                    "{} {}\n{}",
                    title.bold().blue(),
                    pos.italic().magenta(),
                    content
                )
            }
        }
        .as_str(),
        width,
    )
}

fn random_entry() -> Entry {
    let rand_value = rand::thread_rng().gen_range(0..100);
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: Option<&str> = {
        if args.len() > 1 {
            Some(args[1].as_str())
        } else {
            None
        }
    };
}
