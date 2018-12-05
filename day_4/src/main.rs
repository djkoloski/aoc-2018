use std::{
    collections::HashMap,
    env,
    fs::File,
    io::prelude::*,
    str::FromStr,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()));
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum Action {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
enum ParseActionError {
    InvalidAction,
    InvalidGuardId,
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => Ok(Action::FallsAsleep),
            "wakes up" => Ok(Action::WakesUp),
            _ => {
                if &s[0..7] == "Guard #" && &s[s.len()-13..] == " begins shift" {
                    Ok(Action::BeginsShift(s[7..s.len()-13].parse::<u32>().map_err(|_| ParseActionError::InvalidGuardId)?))
                } else {
                    Err(ParseActionError::InvalidAction)
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Event {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    action: Action,
}

#[derive(Debug)]
enum ParseEventError {
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    InvalidHour,
    InvalidMinute,
    ParseActionError(ParseActionError),
}

impl From<ParseActionError> for ParseEventError {
    fn from(error: ParseActionError) -> Self {
        ParseEventError::ParseActionError(error)
    }
}

impl FromStr for Event {
    type Err = ParseEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Event {
            year: s[1..5].parse::<u32>().map_err(|_| ParseEventError::InvalidYear)?,
            month: s[6..8].parse::<u32>().map_err(|_| ParseEventError::InvalidMonth)?,
            day: s[9..11].parse::<u32>().map_err(|_| ParseEventError::InvalidDay)?,
            hour: s[12..14].parse::<u32>().map_err(|_| ParseEventError::InvalidHour)?,
            minute: s[15..17].parse::<u32>().map_err(|_| ParseEventError::InvalidMinute)?,
            action: s[19..].parse::<Action>()?,
        })
    }
}

fn calculate_guard_minutes(data: &str) -> HashMap<u32, [u32;60]> {
    let mut events: Vec<_> = data.lines().map(|l| l.parse::<Event>().unwrap()).collect();
    events.sort_unstable();

    let mut guard_minutes = HashMap::<u32, [u32;60]>::new();

    let mut guard_id: u32 = 0;
    let mut asleep_minute: u32 = 0;
    for event in events {
        match event.action {
            Action::BeginsShift(id) => guard_id = id,
            Action::FallsAsleep => asleep_minute = event.minute,
            Action::WakesUp => {
                let mut minutes = guard_minutes.entry(guard_id).or_insert([0u32;60]);
                for i in asleep_minute..event.minute {
                    minutes[i as usize] += 1;
                }
            }
        }
    }

    guard_minutes
}

fn part_1(data: &str) -> u32 {
    let guard_minutes = calculate_guard_minutes(data);

    let (&result_id, &result_minutes) = guard_minutes.iter().max_by_key(|(_, minutes)| minutes.iter().sum::<u32>()).unwrap();
    let result_minute = result_minutes.iter().enumerate().max_by_key(|(_, &time_asleep)| time_asleep).unwrap().0 as u32;

    result_id * result_minute
}

fn part_2(data: &str) -> u32 {
    let guard_minutes = calculate_guard_minutes(data);

    let (&result_id, &result_minutes) = guard_minutes.iter().max_by_key(|(_, minutes)| minutes.iter().max()).unwrap();
    let result_minute = result_minutes.iter().enumerate().max_by_key(|(_, &time_asleep)| time_asleep).unwrap().0 as u32;

    result_id * result_minute
}