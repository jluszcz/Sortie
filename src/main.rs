use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::{Arg, Command};

fn parse_args() -> Result<Vec<String>> {
    let matches = Command::new("sortie")
        .version("0.1")
        .author("Jacob Luszcz")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .conflicts_with("in")
                .help("Input file."),
        )
        .arg(
            Arg::new("in")
                .short('i')
                .long("in")
                .conflicts_with("file")
                .num_args(1..)
                .help("Input values."),
        )
        .get_matches();

    let file = matches.get_one::<String>("file");
    if let Some(file) = file {
        let mut result = Vec::new();
        for line in BufReader::new(File::open(PathBuf::from_str(file)?)?).lines() {
            result.push(line?);
        }
        return Ok(result);
    }

    if let Some(inputs) = matches.get_many::<String>("in") {
        let mut result = Vec::new();
        for input in inputs {
            result.push(input.to_string());
        }
        return Ok(result);
    }

    Ok(Vec::new())
}

fn sort(input: &mut [String]) {
    input.sort_by_key(|s| {
        if s.starts_with("The ") || s.starts_with("the ") {
            let (_, s) = s.split_at(4);
            s
        } else {
            s
        }
            .to_string()
    });
}

fn main() -> Result<()> {
    let mut input = parse_args()?;
    sort(&mut input);

    for i in input {
        println!("{i}");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn sort() -> Result<()> {
        let mut input = Vec::new();
        for i in ["foo", "bar", "baz"] {
            input.push(format!("{i}"));
            input.push(format!("the {i}"));
            input.push(format!("The {i}"));
            input.push(format!("the{i}"));
        }

        input.shuffle(&mut thread_rng());

        let mut output = input.clone();
        super::sort(&mut output);

        for (i, val) in output.into_iter().enumerate() {
            match i {
                0..3 => assert!(val.contains("bar")),
                3..6 => assert!(val.contains("baz")),
                6..9 => assert!(val.contains("foo")),
                _ => assert!(val.starts_with("the")),
            }
        }

        Ok(())
    }
}
