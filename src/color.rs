use termion::color;

pub fn get_color(color: &str) -> Box<dyn color::Color> {
    let lower_cased_color = color.to_ascii_lowercase();
    println!("{}", lower_cased_color);
    match lower_cased_color.as_str() {
        "red" => Box::new(color::Red),
        "blue" => Box::new(color::Blue),
        "green" => Box::new(color::Green),
        "cyan" => Box::new(color::Cyan),
        "black" => Box::new(color::Black),
        "yellow" => Box::new(color::Yellow),
        "white" => Box::new(color::White),
        "magenta" => Box::new(color::Magenta),
        "lightblack" => Box::new(color::LightBlack),
        "lightblue" => Box::new(color::LightBlue),
        "lightcyan" => Box::new(color::LightCyan),
        "lightred" => Box::new(color::LightRed),
        "lightwhite" => Box::new(color::LightWhite),
        "lightyellow" => Box::new(color::LightYellow),
        _ => Box::new(color::White),
    }
}
