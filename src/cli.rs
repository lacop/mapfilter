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
        long = "max",
        default_value_t = 100,
        help = "Max number of results to show"
    )]
    pub max_results: u64,

    #[clap(short = 'n', long, help = "Filter name tag (if present) by regex")]
    pub name: Option<String>,

    #[clap(short = 't', long, help = "Filter by key=value tag")]
    pub tag_value: Vec<String>,

    #[clap(short = 'r', long, help = "Filter by regex=regex tag")]
    pub tag_regex: Vec<String>,

    #[clap(
        short = 'f',
        long,
        help = "Filter by regex=regex tag, using fancy-regex"
    )]
    pub tag_fancy_regex: Vec<String>,

    #[clap(short = 'l', long, help = "Filter by lat,lon,distance (in meters)")]
    pub lat_lon_distance: Option<String>,

    #[clap(
        long,
        default_value_t = String::from("^(name:.*|alt_name.*|old_name:.*|is_in:.*|wikidata|wikipedia|wikimedia.*|admin_level)$"),
        help = "Regex which tag names to exclude from printing")]
    pub hidden_tags: String,
}
