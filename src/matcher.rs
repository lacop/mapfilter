use crate::cli::Cli;
use color_eyre::eyre::{eyre, Result, WrapErr};
use osmpbf::Element;
use regex::Regex;

#[derive(Debug)]
pub struct Matcher {
    name: Option<Regex>,
    tag_value: Vec<(String, String)>,
}

fn make_regex(pattern: &Option<String>) -> Result<Option<Regex>> {
    pattern
        .as_ref()
        .map(|p| Regex::new(p))
        .transpose()
        .wrap_err("Regex parsing failed")
}

enum TagMatch {
    // Found the tag we need, and it was a match. Stop.
    FoundMatching,
    // Found the tag we need. Wasn't a match but we know we can stop.
    FoundMismatching,
    // Didn't find, keep looking.
    NotFound,
}

// Returns true if provided f returns FoundMatching for any tag. Otherwise false.
fn find_tag_match<FN>(element: &Element, mut f: FN) -> bool
where
    FN: for<'a> FnMut(&'a str, &'a str) -> TagMatch,
{
    match element {
        Element::DenseNode(dn) => {
            for (k, v) in dn.tags() {
                match f(k, v) {
                    TagMatch::FoundMatching => return true,
                    TagMatch::FoundMismatching => return false,
                    TagMatch::NotFound => {}
                }
            }
        }
        // TODO make this generic
        _ => {}
    }
    false
}

impl Matcher {
    pub fn from_cli(args: &Cli) -> Result<Self> {
        Ok(Matcher {
            name: make_regex(&args.name)?,
            tag_value: args
                .tag_value
                .iter()
                .map(|kv| {
                    kv.split_once('=')
                        .map(|(k, v)| (k.to_owned(), v.to_owned()))
                        .ok_or(eyre!("Must be key=value pair"))
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn matches(&self, element: &Element) -> bool {
        // Name regex.
        if let Some(regex) = &self.name {
            if !find_tag_match(element, |k, v| {
                if k == "name" {
                    if regex.is_match(v) {
                        TagMatch::FoundMatching
                    } else {
                        TagMatch::FoundMismatching
                    }
                } else {
                    TagMatch::NotFound
                }
            }) {
                return false;
            }
        }

        // Literal key=value tags.
        for (expected_k, expected_v) in &self.tag_value {
            if !find_tag_match(element, |k, v| {
                if k == expected_k {
                    if v == expected_v {
                        TagMatch::FoundMatching
                    } else {
                        TagMatch::FoundMismatching
                    }
                } else {
                    TagMatch::NotFound
                }
            }) {
                return false;
            }
        }

        // Nothing rejected, so it is a match.
        true
    }
}
