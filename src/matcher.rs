use crate::{cli::Cli, element::element_lat_lon};
use color_eyre::eyre::{eyre, Result, WrapErr};
use geoutils::Location;
use osmpbf::Element;
use regex::Regex;

#[derive(Debug)]
pub struct Matcher {
    pub name: Option<Regex>,
    pub tag_value: Vec<(String, String)>,
    pub tag_regex: Vec<(Regex, Regex)>,
    pub tag_fancy_regex: Vec<(fancy_regex::Regex, fancy_regex::Regex)>,
    pub lat_lon_distance: Option<(f64, f64, f64)>,
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
            tag_regex: args
                .tag_regex
                .iter()
                .map(|kv| {
                    kv.split_once('=')
                        .ok_or(eyre!("Must be regex=regex pair"))
                        .and_then(|(k, v)| Ok((Regex::new(k)?, Regex::new(v)?)))
                })
                .collect::<Result<Vec<_>>>()?,
            tag_fancy_regex: args
                .tag_fancy_regex
                .iter()
                .map(|kv| {
                    kv.split_once('=')
                        .ok_or(eyre!("Must be regex=regex pair"))
                        .and_then(|(k, v)| {
                            Ok((fancy_regex::Regex::new(k)?, fancy_regex::Regex::new(v)?))
                        })
                })
                .collect::<Result<Vec<_>>>()?,
            lat_lon_distance: args
                .lat_lon_distance
                .as_ref()
                .map(|s| {
                    let parts = s.split(',').collect::<Vec<_>>();
                    if parts.len() != 3 {
                        return Err(eyre!("Need three comma-separated values"));
                    }
                    Ok((parts[0].parse()?, parts[1].parse()?, parts[2].parse()?))
                })
                .transpose()?,
        })
    }

    pub fn matches(&self, element: &Element) -> bool {
        // Distance.
        if let Some((query_lat, query_lon, distance)) = self.lat_lon_distance {
            if let Some((lat, lon)) = element_lat_lon(element) {
                let element_location = Location::new(lat, lon);
                let query_location = Location::new(query_lat, query_lon);
                if element_location
                    .haversine_distance_to(&query_location)
                    .meters()
                    > distance
                {
                    return false;
                }
            } else {
                return false;
            }
        }

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

        // Regex=regex tags.
        for (regex_k, regex_v) in &self.tag_regex {
            if !find_tag_match(element, |k, v| {
                if regex_k.is_match(k) {
                    if regex_v.is_match(v) {
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

        // Regex=regex tags, fancy regex.
        for (regex_k, regex_v) in &self.tag_fancy_regex {
            if !find_tag_match(element, |k, v| {
                if regex_k.is_match(k).unwrap_or(false) {
                    if regex_v.is_match(v).unwrap_or(false) {
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
