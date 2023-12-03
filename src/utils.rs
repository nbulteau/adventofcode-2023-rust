// Desc: Utility functions for the project
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

fn green<T: std::fmt::Display>(any: T) -> String {
    format!("\x1B[32m{}\x1B[0m", any)
}

fn blue<T: std::fmt::Display>(any: T) -> String {
    format!("\x1B[34m{}\x1B[0m", any)
}

// A struct to hold the value and the duration
struct TimedValue<T> {
    value: T,
    duration: Duration,
}

// The function that measures the execution time of a closure
fn measure_timed_value<T, F>(block: F) -> TimedValue<T>
    where
        F: FnOnce() -> T,
{
    let start = Instant::now();
    let value = block();
    let duration = start.elapsed();

    TimedValue { value, duration }
}

// Pretty print function
fn pretty_print<T: std::fmt::Display>(message: &str, timed_response: TimedValue<T>) {
    println!(
        "{} : {} ({} ms)",
        message,
        blue(&timed_response.value),
        green(timed_response.duration.as_millis())
    );
}

// Pretty print for part one
pub fn pretty_print_part_one<T, F>(lambda: F)
    where
        T: std::fmt::Display,
        F: FnOnce() -> T,
{
    pretty_print("Part one answer", measure_timed_value(lambda));
}

// Pretty print for part two
pub fn pretty_print_part_two<T, F>(lambda: F)
    where
        T: std::fmt::Display,
        F: FnOnce() -> T,
{
    pretty_print("Part two answer", measure_timed_value(lambda));
}

/// Get the input for a given day.
pub fn get_day_input(day: u8) -> Result<String, io::Error> {
    fs::read_to_string(Path::new(&"resources").join(format!("day{:02}.txt", day)))
}