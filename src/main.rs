mod gen_entries;
use gen_entries::ENTRIES as jargon_entries;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn match_entry(query: &str, threshold: i64) -> Option<[&str; 3]> {
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

fn main() {
    if let Some(s) = match_entry("zorkmid", 0) {
        let desc = s[2];

        println!("{}", desc);
    } else {
        println!("That didn't work out")
    };
}
