extern crate clap;
extern crate termion;

use std::io::{stdout, Write};
use std::process::Command;
use std::thread;
use std::time;

use termion::color;
use termion::raw::IntoRawMode;
use termion::style;

use seer::cli::create_cli;
use seer::utils::*;

use seer::color::get_color;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termion::input::TermRead;

// One environment variable for timer color
// Another environment variable for task color
use std::env;

fn start_handler(stop_flag: Arc<AtomicBool>) {
    let stdin = std::io::stdin();
    thread::spawn(move || {
        for _ in stdin.keys() {
            stop_flag.fetch_or(true, Ordering::SeqCst);
        }
    });
}

fn get_env_variables() -> (String, String) {
    let timer_color = match env::var("TIMER_COLOR") {
        Ok(val) => val,
        Err(_) => String::from("None"),
    };
    let task_color = match env::var("TASK_COLOR") {
        Ok(val) => val,
        Err(_) => String::from("None"),
    };

    (timer_color, task_color)
}

// MacOS target implementation
#[cfg(target_os = "macos")]
fn notify_task_done(task: String) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "display notification \"Task {} has ended\" sound name \"default\"",
            task
        ))
        .output()
        .expect("osacript command failed");
}

#[cfg(target_os = "linux")]
fn notify_task_done() {
    // TODO
}

fn main() {
    let matches = create_cli().get_matches();

    let stop_flag = Arc::new(AtomicBool::new(false));
    start_handler(stop_flag.clone());

    let task = matches
        .value_of("project")
        .expect("Project not found")
        .to_string();
    let timer = matches
        .value_of("timer")
        .expect("Timer not found")
        .to_string();
    let (timer_color, task_color) = get_env_variables();

    // TODO: Ugly, need to refactor this
    let c = get_color(timer_color.as_str());
    let timer_color = color::Fg(&*c);
    let c = get_color(task_color.as_str());
    let task_color = color::Fg(&*c);

    let mut seconds = to_seconds(timer).unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();

    loop {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }
        let (horizontal, vertical) = termion::terminal_size().unwrap();
        let formatted_time = to_hours(seconds);

        let offset: u16 = (task.len() / 2) as u16;
        write!(
            stdout,
            "{}{}{}{}{}{}",
            termion::cursor::Goto(horizontal / 2 - offset + 3, vertical / 2 - 2),
            termion::style::Bold,
            task_color,
            task,
            color::Fg(color::Reset),
            style::Reset,
        )
        .unwrap();
        stdout.flush().unwrap();

        // hours
        write!(stdout, "{}", timer_color).unwrap();
        formatted_time[0].render((horizontal / 2 - 22, vertical / 2), &mut stdout);
        formatted_time[1].render((horizontal / 2 - 14, vertical / 2), &mut stdout);
        // minutes
        formatted_time[2].render((horizontal / 2 - 4, vertical / 2), &mut stdout);
        formatted_time[3].render((horizontal / 2 + 4, vertical / 2), &mut stdout);
        // seconds
        formatted_time[4].render((horizontal / 2 + 14, vertical / 2), &mut stdout);
        formatted_time[5].render((horizontal / 2 + 22, vertical / 2), &mut stdout);

        thread::sleep(time::Duration::from_secs(1));
        // Clearing tty device
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();
        seconds -= 1;

        if seconds == 0 {
            notify_task_done(task);
            write!(stdout, "{}{}", color::Fg(color::Reset), termion::clear::All).unwrap();
            break;
        }
    }
}
