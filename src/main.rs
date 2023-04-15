use std::env;
use std::io;

fn main() {
    let config = config().expect("Invalid arguments");
    pring_header(&config);

    let input = io::stdin();
}

type Result<T> = std::result::Result<T, &'static str>;

struct Config {
    header: bool,
    columns: Vec<(String, String)>, // name, path
}

fn config() -> Result<Config> {
    let mut arg_counter = -1;
    let mut header = true;
    let mut columns = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        arg_counter += 1;
        let argtmp: &str = &arg;
        match &argtmp {
            &"--no-head" => header = false,
            &"-c" => columns.push((
                args.next().ok_or("Not found key argument")?,
                args.next().ok_or("Not found path argument")?
            )),
            _ => columns.push((arg_counter.to_string(), arg)),
        }
    }
    Ok(Config {
        header,
        columns,
    })
}

fn pring_header(config: &Config) {
    if config.header {
        let header = config.columns
            .iter()
            .map(|el|&el.0)
            .map(|el|el.as_str())
            .collect::<Vec<_>>()
            .join(",");
        println!("{}", header);
    }
}

