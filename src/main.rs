use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use console::Term;
use osmpbf::ElementReader;

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

    println!("Using OSM PBF file from '{}'", args.map_file);
    let map_reader = ElementReader::from_path(args.map_file)?;

    // Need sync channel to make it Sync + Send for the closure.
    // Bottleneck should be the matching, not collecting & printing, but
    // give it some buffer anyway.
    let (tx, rx) = sync_channel::<OwnedElement>(10);

    let consumer_thread = thread::spawn(move || {
        let mut count = 0;
        let term = Term::stdout();
        for element in rx {
            //println!("{element:?}");
            element.print(&term).expect("Priting failed");
            count += 1;
            if count > args.max_results {
                // TODO print warning we missed some results?
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

    // TODO fancier printing
    // TODO show excluded result count
    println!(
        "Results: {} / {}",
        matches.separate_with_underscores(),
        total.separate_with_underscores()
    );

    Ok(())
}
