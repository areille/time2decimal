#[macro_use]
extern crate clap;

use std::process;

pub fn convert(hours: u32, minutes: u32, seconds: u32) -> f32 {
    hours as f32 + minutes as f32 / 60. + seconds as f32 / 3600.
}

fn main() {
    let matches = clap_app!(t2d =>
        (version: "0.1.0")
        (author: "areille <areille@tuta.io>")
        (about: "Converts time to decimal")
        (@arg HOURS: +required "Hours")
        (@arg MINUTES: +required "Minutes")
        (@arg SECONDS: "Seconds")
    )
    .get_matches();

    let hours = matches
        .value_of("HOURS")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing <HOURS> argument : {}", err);
            process::exit(1);
        });
    let minutes = matches
        .value_of("MINUTES")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing <MINUTES> argument : {}", err);
            process::exit(1);
        });
    let seconds = matches
        .value_of("SECONDS")
        .unwrap_or("00")
        .parse::<u32>()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing <SECONDS> argument : {}", err);
            process::exit(1);
        });

    let hours_in_decimal = convert(hours, minutes, seconds);
    let time_in_days = hours_in_decimal / 7.5;

    println!(
        "{}:{}:{} in decimal is {} hours, or {} days",
        hours, minutes, seconds, hours_in_decimal, time_in_days,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_1() {
        assert_eq!(convert(1, 0, 0), 1.0);
    }
    #[test]
    fn convert_2() {
        assert_eq!(convert(12, 20, 53), 12.348055);
    }
}
