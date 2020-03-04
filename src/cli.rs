use clap::{App, Arg};

pub fn create_cli<'a, 'b>() -> App<'a, 'b> {
    let matches = App::new("Seer")
        .version("0.0.1")
        .author("Ingi Þór Sigurðsson <ingisigurds@gmail.com>")
        .about("Easily manage work time on projects with detailed reports")
        .arg(
            Arg::with_name("project")
                .help("What task you want to create / start etc.")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("timer")
                .index(2)
                .required(true)
                .takes_value(true),
        );

    matches
}
