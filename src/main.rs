//! # Subtitle-Sync
//!
//! A small CLI Utility that helps tackling with out-of-sync subtitles
//!
//! # How to use
//! Let's say you have a subtitle file `some-awesome-show-S01E01.srt`. But when you play the video
//! with it, you notice the subtitle slightly goes out of sync; without being able to adjust it
//! by a simple offset.
//!
//! This library aims at dealing with this kind of trouble.
//! Firstly, open the video and find a sentence (the later in the video the better). Write down
//! its time code. For instance, 00:43:50,200
//!
//! Secondly, open the .srt file and find the time code of this same sentence. For
//! instance, 00:40:50,652
//!
//! You can now use this script to adapt the file, by running in a terminal :
//! ```bash
//! $ subtitle-sync --input some-awesome-show-S01E01.srt --from 00:40:50,652 --to 00:43:50,200
//! Computed ratio is 1.0732654
//! Sync finished in 6ms. Wrote to file new-some-awesome-show-S01.srt
//! ```


extern crate clap;
use clap::{Arg, App};
use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::time::SystemTime;

// TimeCode Struct and functions

struct TimeCode {
    h: u32,
    m: u32,
    s: u32,
    ms: u32
}

impl TimeCode {
    fn to_ms(&self) -> u32 {
        return (((self.h * 60 + self.m) * 60) + self.s) * 1000 + self.ms;
    }

    fn to_string(&self) -> String {
        let h = format!("{:0>2}", self.h);
        let m = format!("{:0>2}", self.m);
        let s = format!("{:0>2}", self.s);
        let ms = format!("{:0>3}", self.ms);
        return format!("{}:{}:{},{}", h, m, s, ms);
    }

    fn sync(&self, ratio: f32) -> String {
        let final_ms = (self.to_ms() as f32 * ratio).trunc() as u32;
        let time_code_sync = TimeCode::from_ms(&final_ms);
        return time_code_sync.to_string();
    }

    fn from_ms(ms_total: &u32) -> TimeCode {
        let h = ms_total / 3600000;
        let m = (ms_total % 3600000) / 60000;
        let s = ((ms_total % 3600000) % 60000) / 1000;
        let ms = ms_total % 1000;
        return TimeCode { h, m, s, ms };
    }
}

impl FromStr for TimeCode {
    type Err = std::num::ParseIntError;

    fn from_str(time_code: &str) -> Result<Self, Self::Err> {
        let h: u32 = u32::from_str(&time_code[0..2])?;
        let m: u32 = u32::from_str(&time_code[3..5])?;
        let s: u32 = u32::from_str(&time_code[6..8])?;
        let ms: u32 = u32::from_str(&time_code[9..12])?;

        Ok(TimeCode { h, m, s, ms })
    }
}

// TimeRange Struct and functions

struct TimeRange {
    beginning: TimeCode,
    end: TimeCode
}

impl TimeRange {
    fn sync(&self, ratio: f32) -> String {
        let beginning_sync = self.beginning.sync(ratio);
        let end_sync = self.end.sync(ratio);
        return format!("{} --> {}", beginning_sync, end_sync);
    }
}

impl FromStr for TimeRange {
    type Err = std::num::ParseIntError;

    fn from_str(time_range: &str) -> Result<Self, Self::Err> {
        let beginning = TimeCode::from_str(&time_range[0..12])?;
        let end = TimeCode::from_str(&time_range[17..29])?;

        Ok(TimeRange { beginning, end })
    }
}

// Main functions

fn main() {
    let start_time = SystemTime::now();

    let matches = App::new("Subtitle-Sync")
        .version("0.1")
        .author("Bastien Huber")
        .about("Transforms UTF-8 encoded .srt files time-codes to sync them")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .value_name("INPUT")
            .required(true)
            .help(".srt file to transform")
            .takes_value(true))
        .arg(Arg::with_name("FROM")
            .short("f")
            .long("from")
            .value_name("FROM")
            .required(true)
            .help("Time code of the .srt file to rely on to compute ratio factor")
            .takes_value(true))
        .arg(Arg::with_name("TO")
            .short("t")
            .long("to")
            .value_name("TO")
            .required(true)
            .help("Time code taken from the video file to sync")
            .takes_value(true))
        .get_matches();

    let path = matches.value_of("INPUT").unwrap();
    let from = matches.value_of("FROM").unwrap();
    let to = matches.value_of("TO").unwrap();

    let ratio = compute_ratio(from, to);
    println!("Computed ratio is {}", ratio);

    let old_file = File::open(path).expect("File not found");
    let new_file_name = format!("new-{}", path);
    let new_file = File::create(&new_file_name).expect("Failed to create output file");

    let reader = BufReader::new(old_file);
    let mut buffered_out = BufWriter::new(new_file);

    reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| buffered_out.write_all(update_line(line.unwrap(), ratio).as_bytes()))
        .collect::<Result<(), _>>()
        .expect("IO Failed");

    let script_duration = SystemTime::now()
        .duration_since(start_time)
        .expect("An error occurred while computing the script execution time");
    println!("Sync finished in {}ms. Wrote to file {}", script_duration.as_millis(), &new_file_name);
}

fn update_line(content: String, ratio: f32) -> String {
    if content.len() >= 29 && content.contains("-->") {
        let initial_time_range = &content[0..29];
        let time_range = TimeRange::from_str(initial_time_range).unwrap();
        let final_time_range = time_range.sync(ratio);
        return content.replace(initial_time_range, &final_time_range) + "\n";
    }
    return content + "\n";
}

fn compute_ratio(from: &str, to:&str ) -> f32 {
    let from_tc = TimeCode::from_str(&from).unwrap();
    let to_tc = TimeCode::from_str(&to).unwrap();
    return (to_tc.to_ms() as f32) / (from_tc.to_ms() as f32);
}
