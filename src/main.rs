use argh::FromArgs;
use serde_json::value;
use std::{error::Error, ffi::OsStr, fs};
use walkdir::WalkDir;

/// usage of pitufo
#[derive(FromArgs)]
pub struct Opts {
    /// follow symbolic links, the default is to not follow.
    #[argh(switch, long = "follow")]
    follow_links: bool,

    /// minify the json, the default is to prettify.
    #[argh(switch)]
    minify: bool,

    /// set the maximum depth to recurse
    #[argh(option, short = 'm', default = "0")]
    max_depth: usize,

    /// the path to search for json files.
    #[argh(option, short = 'p')]
    path: String,
}

fn main() {
    let options: Opts = argh::from_env();

    let mut walk = WalkDir::new(options.path);

    walk = walk.follow_links(options.follow_links);

    if options.max_depth > 0 {
        walk = walk.max_depth(options.max_depth);
    }

    for entry in walk
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(OsStr::to_str) == Some("json"))
    {
        if let Err(e) = process_file(&entry, options.minify) {
            eprintln!("error {} {}", e, entry.path().display());
        }
    }
}

fn process_file(entry: &walkdir::DirEntry, minify: bool) -> Result<(), Box<dyn Error>> {
    let content = fs::read(entry.path())?;

    // Attempt to fix the bom error
    let without_bom = if content.starts_with(b"\xEF\xBB\xBF") {
        &content[3..]
    } else if (content.starts_with(b"\xFF\xFE")) || (content.starts_with(b"\xFE\xFF")) {
        &content[2..]
    } else {
        &content
    };

    let json: value::Value = serde_json::from_slice(without_bom)?;
    let output = if minify {
        serde_json::to_vec(&json)?
    } else {
        serde_json::to_vec_pretty(&json)?
    };
    fs::write(entry.path(), output)?;

    Ok(())
}
