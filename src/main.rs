use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use console::{style, Term};
use osmpbf::ElementReader;
use regex::Regex;
use std::sync::mpsc::sync_channel;
use std::thread;
use thousands::Separable;

use mapfilter::cli::Cli;
use mapfilter::element::OwnedElement;
use mapfilter::matcher::Matcher;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse CLI arguments and construct Matcher out of them.
    let args = Cli::parse();
    if args.debug {
        println!("{args:#?}");
    }
    let matcher = Matcher::from_cli(&args)?;
    if args.debug {
        println!("{matcher:#?}");
    }

    let hidden_tags = Regex::new(&args.hidden_tags)?;
    let query_lat_lon = matcher.lat_lon_distance.map(|(lat, lon, _)| (lat, lon));

    println!("Using OSM PBF file from '{}'", args.map_file);
    let map_reader = ElementReader::from_path(args.map_file)?;

    // Need sync channel to make it Sync + Send for the closure.
    // Bottleneck should be the matching, not collecting & printing, but
    // give it some buffer anyway.
    let (tx, rx) = sync_channel::<OwnedElement>(10);

    let consumer_thread = thread::spawn(move || {
        let term = Term::stdout();
        let mut count = 0;
        for element in rx {
            count += 1;
            element
                .print(&term, count, query_lat_lon, &hidden_tags)
                .expect("Priting failed");
            if count > args.max_results {
                term.write_line(
                    &style("✂️ Reached output limit, not showing more")
                        .red()
                        .to_string(),
                )
                .expect("Printing failed");
                break;
            }
        }
    });

    let (matches, total) = map_reader.par_map_reduce(
        move |element| {
            if matcher.matches(&element) {
                // Ignore send error if consumer closed the channel early.
                let _ = tx.send(element.into());
                (1, 1)
            } else {
                (0, 1)
            }
        },
        || (0, 0),
        |(a, b), (c, d)| (a + c, b + d),
    )?;
    consumer_thread.join().map_err(|_| eyre!("Join failed"))?;

    let term = Term::stdout();
    term.write_line("")?;

    term.write_line(&format!(
        "{}: {} / {}: {} / {}: {}",
        style("Total nodes").bold(),
        total.separate_with_underscores(),
        style("Filtered to").bold(),
        matches.separate_with_underscores(),
        style("Displayed").bold(),
        args.max_results.min(matches).separate_with_underscores()
    ))?;

    Ok(())
}
