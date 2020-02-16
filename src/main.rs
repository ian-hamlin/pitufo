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

    /// verbose mode, output the path of every file found,
    /// defaults to silent
    #[argh(switch)]
    verbose: bool,

    /// minify the json, the default is to prettify.
    #[argh(switch)]
    minify: bool,

    /// look for leading BOM in json files and remove if found,
    /// the default is to take the file as-is.
    #[argh(switch)]
    bom: bool,

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
        if let Err(e) = process_file(&entry, options.minify, options.bom) {
            eprintln!("error {} {}", e, entry.path().display());
        } else if options.verbose {
            println!("{}", entry.path().display());
        }
    }
}

fn process_file(
    entry: &walkdir::DirEntry,
    minify: bool,
    look_at_bom: bool,
) -> Result<(), Box<dyn Error>> {
    let content = fs::read(entry.path())?;

    // Attempt to fix the bom error
    let content_bomed = bom_check(&content, look_at_bom);
    let json: value::Value = serde_json::from_slice(content_bomed)?;

    let output = if minify {
        serde_json::to_vec(&json)?
    } else {
        serde_json::to_vec_pretty(&json)?
    };

    fs::write(entry.path(), output)?;

    Ok(())
}

fn bom_check(content: &Vec<u8>, look_at_bom: bool) -> &[u8] {
    if look_at_bom && content.starts_with(b"\xEF\xBB\xBF") {
        &content[3..]
    } else if look_at_bom
        && ((content.starts_with(b"\xFF\xFE")) || (content.starts_with(b"\xFE\xFF")))
    {
        &content[2..]
    } else {
        &content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_remove_bom_utf8() {
        let content: Vec<u8> = vec![239, 187, 191, 1];
        let result = bom_check(&content, true);

        assert_eq!(vec![1], result);
    }

    #[test]
    fn should_not_remove_bom_utf8() {
        let content: Vec<u8> = vec![239, 187, 191, 1];
        let result = bom_check(&content, false);

        assert_eq!(vec![239, 187, 191, 1], result);
    }

    #[test]
    fn should_remove_bom_utf16le() {
        let content: Vec<u8> = vec![255, 254, 1];
        let result = bom_check(&content, true);

        assert_eq!(vec![1], result);
    }

    #[test]
    fn should_not_remove_bom_utf16le() {
        let content: Vec<u8> = vec![255, 254, 1];
        let result = bom_check(&content, false);

        assert_eq!(vec![255, 254, 1], result);
    }

    #[test]
    fn should_remove_bom_utf16be() {
        let content: Vec<u8> = vec![254, 255, 1];
        let result = bom_check(&content, true);

        assert_eq!(vec![1], result);
    }

    #[test]
    fn should_not_remove_bom_utf16lbe() {
        let content: Vec<u8> = vec![254, 255, 1];
        let result = bom_check(&content, false);

        assert_eq!(vec![254, 255, 1], result);
    }
}
