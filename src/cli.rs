use clap::{App, Arg};

pub fn run() -> (String, String, u8, bool, bool) {
    let matches = App::new("silqfmt")
        .version(env!("CARGO_PKG_VERSION"))
        .author("MamoruDS <mamoruds.io@gmail.com>")
        .about("Formatter for Silq")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("input-file")
                .help("Path of input file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tab-size")
                .long("tab-size")
                .value_name("tab-size")
                .help("Size of indent")
                .required(false)
                .default_value("4")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("output-file")
                .help("Path of output file")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hardtab")
                .long("hardtab")
                .help("Using hardtabs instead of spaces"),
        )
        .arg(
            Arg::with_name("unicode")
                .long("unicode")
                .help("Using unicode symbols"),
        )
        .get_matches();

    let input_path: String = match matches.value_of("input") {
        Some(v) => String::from(v),
        None => panic!(),
    };

    let output_path: String = match matches.value_of("output") {
        Some(v) => String::from(v),
        None => String::new(),
    };

    let tab_size: u8 = matches.value_of("tab-size").unwrap().parse().unwrap();

    let hardtab: bool = match matches.occurrences_of("hardtab") {
        0 => false,
        _ => true,
    };

    let unicode_symbol: bool = match matches.occurrences_of("hardtab") {
        0 => false,
        _ => true,
    };

    (input_path, output_path, tab_size, hardtab, unicode_symbol)
}
