use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(help = "Path to OSM PBF map file")]
    pub map_file: String,

    #[clap(long, help = "Debug mode")]
    pub debug: bool,

    #[clap(
        short = 'm',
        long,
        default_value_t = 100,
        help = "Max number of results to show"
    )]
    pub max_results: u64,

    #[clap(short = 'n', long, help = "Filter name tag (if present) by regex")]
    pub name: Option<String>,

    #[clap(short = 't', long, help = "Filter by key=value tag")]
    pub tag_value: Vec<String>,
    // DONE
    // - name regex
    // - tag string + string, direct match
    // TODO:
    // - tag regex + regex
    // - tag fancyregex + fancyregex?
    // - lat lon (distance from point?)
}