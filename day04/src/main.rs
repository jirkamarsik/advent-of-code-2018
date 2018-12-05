extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    date_time: NaiveDateTime,
    event_type: EventType,
}

type GuardID = u32;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    ShiftChange(GuardID),
    FallingAsleep,
    WakingUp,
}

struct TimeInterval {
    from: NaiveDateTime,
    to: NaiveDateTime,
}

#[derive(PartialEq, Eq, Hash)]
struct Opportunity {
    guard_id: GuardID,
    minute: u32,
}

fn parse_input() -> Vec<Event> {
    let line_parser = Regex::new(
        r"(?x)
\[ (?P<date_time>\d{4}-\d{2}-\d{2} \s+ \d{2}:\d{2}) \]
\s+
(?: (?P<shift_change> Guard\ \#(?P<guard_id>\d+)\ begins\ shift)
  | (?P<falling_asleep> falls\ asleep)
  | (?P<waking_up> wakes\ up))",
    )
    .unwrap();

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut events = vec![];
    for line in buf.lines() {
        let cap = line_parser.captures(line).expect("Parse error in input!");
        let date_time = NaiveDateTime::parse_from_str(&cap["date_time"], "%Y-%m-%d %H:%M")
            .expect("Parse error in date/time!");
        let event_type = if cap.name("shift_change").is_some() {
            EventType::ShiftChange(cap["guard_id"].parse().expect("Parse error in guard ID!"))
        } else if cap.name("falling_asleep").is_some() {
            EventType::FallingAsleep
        } else {
            EventType::WakingUp
        };
        events.push(Event {
            date_time,
            event_type,
        });
    }
    events.sort_unstable();
    events
}

fn record_naps(events: &Vec<Event>) -> HashMap<GuardID, Vec<TimeInterval>> {
    let mut sleep_times = HashMap::new();
    let mut current_guard = None;
    let mut fell_asleep = None;

    for event in events {
        match event.event_type {
            EventType::ShiftChange(new_guard) => {
                assert_eq!(None, fell_asleep);
                current_guard = Some(new_guard);
                fell_asleep = None;
            }
            EventType::FallingAsleep => {
                assert_eq!(None, fell_asleep);
                assert!(current_guard.is_some());
                fell_asleep = Some(event.date_time);
            }
            EventType::WakingUp => {
                assert!(fell_asleep.is_some());
                assert!(current_guard.is_some());
                let nap = TimeInterval {
                    from: fell_asleep.unwrap(),
                    to: event.date_time,
                };
                sleep_times
                    .entry(current_guard.unwrap())
                    .or_insert_with(|| Vec::new())
                    .push(nap);
                fell_asleep = None;
            }
        }
    }

    sleep_times
}

fn find_best_opportunity_s1(events: &Vec<Event>) -> Opportunity {
    let guards_naps = record_naps(events);

    let sleepiest_guard = *guards_naps
        .iter()
        .max_by_key(|&(_, naps)| {
            naps.iter()
                .map(|&TimeInterval { from, to }| (to - from).num_minutes())
                .sum::<i64>()
        })
        .unwrap()
        .0;

    let mut sleepy_minutes = [0; 60];
    for nap in &guards_naps[&sleepiest_guard] {
        for sleepy_minute in nap.from.minute()..nap.to.minute() {
            sleepy_minutes[sleepy_minute as usize] += 1;
        }
    }

    let sleepiest_minute = sleepy_minutes
        .iter()
        .enumerate()
        .max_by_key(|(_, &s)| s)
        .unwrap()
        .0;
    Opportunity {
        guard_id: sleepiest_guard,
        minute: sleepiest_minute as u32,
    }
}

fn find_best_opportunity_s2(events: &Vec<Event>) -> Opportunity {
    let guards_naps = record_naps(events);

    let mut opportunities = HashMap::new();
    for (guard_id, naps) in guards_naps {
        for nap in naps {
            for minute in nap.from.minute()..nap.to.minute() {
                *opportunities
                    .entry(Opportunity { guard_id, minute })
                    .or_insert(0) += 1;
            }
        }
    }

    opportunities
        .into_iter()
        .max_by_key(|&(_, score)| score)
        .unwrap()
        .0
}

fn main() {
    let events = parse_input();
    // Need to learn trait objects and/or closures to refactor below.
    let best_opportunity_s1 = find_best_opportunity_s1(&events);
    println!(
        "The best time to go in using Strategy 1 is when guard #{} is on duty, at 00:{}! (Answer: {})",
        best_opportunity_s1.guard_id,
        best_opportunity_s1.minute,
        best_opportunity_s1.guard_id * best_opportunity_s1.minute
    );
    let best_opportunity_s2 = find_best_opportunity_s2(&events);
    println!(
        "The best time to go in using Strategy 2 is when guard #{} is on duty, at 00:{}! (Answer: {})",
        best_opportunity_s2.guard_id,
        best_opportunity_s2.minute,
        best_opportunity_s2.guard_id * best_opportunity_s2.minute
    );
}
