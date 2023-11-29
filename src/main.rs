use std::fmt::Write;
use std::thread;

use chrono::{DateTime, Duration, Local};
use clap::{arg, command, value_parser};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    // setup cli parser
    let cli = command!()
        .arg(arg!([title] "The name of the task to track").required(true))
        .arg(
            arg!([duration] "The duration of the task in minutes (default=25m)")
                .value_parser(value_parser!(i64)),
        )
        .get_matches();

    // parse arguments
    let title = cli.get_one::<String>("title").unwrap().to_string();
    let duration: Duration = Duration::minutes(*cli.get_one::<i64>("duration").unwrap_or(&25));

    // setup view
    let start: DateTime<Local> = Local::now();
    let end = start + duration;
    let pb = build_view(start, duration, title);

    // main loop
    let mut now = start;
    while now < end {
        let elapsed = now - start;
        pb.set_position(elapsed.num_seconds().try_into().unwrap());
        thread::sleep(std::time::Duration::from_millis(500));
        now = Local::now();
    }
    pb.finish();
}

fn build_view(start: DateTime<Local>, duration: Duration, title: String) -> ProgressBar {
    let pb = ProgressBar::new(duration.num_seconds().try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {title} {duration} {start} {elapsed_precise} [{wide_bar:.cyan/blue}]",
        )
        .unwrap()
        .with_key("start", move |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", start.format("%H:%M").to_string()).unwrap()
        })
        .with_key("title", move |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", title).unwrap()
        })
        .with_key(
            "duration",
            move |_state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{}m", duration.num_minutes()).unwrap()
            },
        )
        .progress_chars("#>-"),
    );
    pb
}
