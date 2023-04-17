use std::env;
use std::io;
use std::io::BufRead;
use json::TypeJson;

fn main() {
    let config = config().expect("Invalid arguments");
    print_header(&config);

    io::stdin().lock().lines()
        .map(|r|r.expect("Unable to read line."))
        .map(|line| map_row(&config, line))
        .for_each(|line| println!("{}", line));
}

type Result<T> = std::result::Result<T, &'static str>;

struct Config {
    header: bool,
    string_escape: bool,
    columns: Vec<(String, String)>, // name, path
}

fn config() -> Result<Config> {
    let mut arg_counter = -1;
    let mut header = true;
    let mut escape = true;
    let mut columns = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        arg_counter += 1;
        let argtmp: &str = &arg;
        match &argtmp {
            &"--no-head" => header = false,
            &"--no-escape" => escape = false,
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
        string_escape: escape,
    })
}

fn print_header(config: &Config) {
    if config.header {
        println!("{}", get_header(config));
    }

}

fn get_header(config: &Config) -> String {
    config.columns
        .iter()
        .map(|el|&el.0)
        .map(|el|el.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn map_row(config: &Config, line: String) -> String {
    let mut chars = line.chars();
    let object = json::parser(&mut chars).expect("Unable to read object from line").into();
    let reader = json::ReaderJson::new(&object);
    config.columns
        .iter()
        .map(|(_, path)| path)
        .map(|path|reader.path(path))
        .map(|obj| {
            if config.string_escape {
                obj.json().to_string()
            } else {
                match obj.json() {
                    TypeJson::Object(_) => String::from("[object_json]"),
                    TypeJson::List(_) => String::from("[list_json]"),
                    TypeJson::Text(txt) => txt.to_string(),
                    TypeJson::Number(n) => n.to_string(),
                    TypeJson::Null => String::from(""),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}
