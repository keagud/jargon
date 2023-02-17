mod gen_entries;
use gen_entries::ENTRIES as jargon_entries;
use gen_entries::NUM_ENTRIES as entries_count;

use std::env;
use std::io;

use colored::Colorize;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rand::Rng;
use textwrap;

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

fn print_entry(e: Entry, opts: textwrap::Options) {
    let entry_str = match e {
        [title, "", content] => format!("{}\n{}", title.bold().blue(), content),
        [title, pos, content] => {
            format!(
                "{} {}\n{}",
                title.bold().blue(),
                pos.italic().magenta(),
                content
            )
        }
    };

    println!("{}", textwrap::fill(entry_str.as_str(), opts));
}

fn random_entry() -> Entry {
    let rand_value = rand::thread_rng().gen_range(0..entries_count);

    jargon_entries[rand_value]
}

fn main() -> io::Result<()> {
    const THRESHOLD: i64 = 0;

    let args: Vec<String> = env::args().collect();

    let query = if args.len() > 1 { Some(&args[1]) } else { None };

    let query_result = match query {
        Some(q) => match_entry(q, THRESHOLD),
        None => Some(random_entry()),
    }
    .ok_or(io::ErrorKind::NotFound)?;

    let opts = textwrap::Options::new(80)
        .initial_indent("  ")
        .subsequent_indent("    ");

    print_entry(query_result, opts);
    Ok(())
}
