use ::std::fs::File;
use clap::{App, Arg};
use std::error::Error;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for (line_num, lines_result) in file.lines().enumerate() {
                    let line = lines_result?;
                    if config.number_lines {
                        if line.is_empty() {
                            println!("{:>6}\t", line_num + 1)
                        } else {
                            println!("{:>6}\t{}", line_num + 1, line);
                        }
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!()
                        } else {
                            println!("{:>6}\t{}", line_num + 1, line);
                        }
                    }
                    
                    else {
                        if line.is_empty() {
                            println!()
                        }else {
                            println!("{}", line)
                        }
                        
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Sylvain Martel")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false)
                .short("b")
                .conflicts_with("number")
                .long("number-nonblank"),
        )
        .arg(
            Arg::with_name("number")
                .help("Number lines")
                .takes_value(false)
                .short("n")
                .long("number"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
