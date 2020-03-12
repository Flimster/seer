extern crate clap;
extern crate termion;

use std::io::{stdout, Write};
use std::thread;
use std::time;

use termion::color;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use seer::cli::create_cli;
use seer::state::{Task, Timer};
use seer::utils::*;

use seer::color::get_color;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use termion::input::TermRead;

// One environment variable for timer color
// Another environment variable for task color
const ONGOING: usize = 1;
const STOPPED: usize = 2;
const QUIT: usize = 3;

// Wait for any stoppage of timer
// Simply stops the timer but does not exit
// If 's' is pressed, then stop the timer, has no effect if timer is stopped
// If 'c' is preseed continue the timer, has no effect if timer is ongoing
fn start_handler(stop_flag: Arc<AtomicUsize>) {
    let stdin = std::io::stdin();
    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                // Boolean logic to determine what state the timer should be in
                Key::Char('s') => {
                    stop_flag.store(STOPPED, Ordering::SeqCst);
                }
                Key::Char('c') => {
                    stop_flag.store(ONGOING, Ordering::SeqCst);
                }
                _ => stop_flag.store(QUIT, Ordering::SeqCst),
            }
        }
    });
}

fn get_state(state: usize) -> String {
    match state {
        ONGOING => String::from("Ongoing"),
        STOPPED => String::from("Stopped"),
        _ => String::from("Quit"),
    }
}


fn main() {
    let matches = create_cli().get_matches();

    let state_machine = Arc::new(AtomicUsize::new(1));
    start_handler(state_machine.clone());

    let task_name = matches
        .value_of("project")
        .expect("Project not found")
        .to_string();
    let timer = matches
        .value_of("timer")
        .expect("Timer not found")
        .to_string();

    let (timer_color, task_color) = get_env_variables();
    let c = get_color(timer_color.as_str());
    let timer_color = color::Fg(&*c);
    let c = get_color(task_color.as_str());
    let task_color = color::Fg(&*c);

    let mut seconds = to_seconds(timer).unwrap();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // Make sure to hide the cursor
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    loop {
        if state_machine.load(Ordering::SeqCst) == QUIT {
            // The cursor is hidden even after the program exists if termion::cursor::Show is not used
            write!(
                stdout,
                "{}{}{}",
                color::Fg(color::Reset),
                termion::clear::All,
                termion::cursor::Show
            )
            .unwrap();
            break;
        }

        let formatted_time = to_hours(seconds);

        let task = Task::new(task_name.clone(), task_color);
        let timer = Timer::new(formatted_time);

        let state = get_state(state_machine.load(Ordering::SeqCst));
        task.render(state, &mut stdout);
        write!(stdout, "{}", timer_color).expect("Failed to write to stdout");
        timer.render(&mut stdout);

        thread::sleep(time::Duration::from_secs(1));

        if state_machine.load(Ordering::SeqCst) == ONGOING {
            write!(stdout, "{}", termion::clear::All).unwrap();
            stdout.flush().unwrap();
            seconds -= 1;

            if seconds == 0 {
                notify_task_done(task_name);
                write!(
                    stdout,
                    "{}{}{}",
                    color::Fg(color::Reset),
                    termion::clear::All,
                    termion::cursor::Show
                )
                .unwrap();
                break;
            }
        }
    }
}
