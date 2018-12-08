use datetime::LocalDate;
use datetime::LocalDateTime;
use datetime::LocalTime;
use datetime::Month;
use datetime::TimePiece;
use measure::measure_and_print;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Action {
  BeginsShift,
  WakesUp,
  FallsAsleep,
}

impl Action {
  fn parse(action: &str) -> Result<Self, String> {
    if action == "wakes up" {
      return Ok(Action::WakesUp);
    } else if action == "falls asleep" {
      return Ok(Action::FallsAsleep);
    }

    let mut error = String::from("Unknown action ");
    error.push_str(action);

    Err(error)
  }
}

#[derive(Debug)]
struct Log {
  moment: LocalDateTime,
  guard: Option<u32>,
  action: Action,
}

#[derive(Copy, Clone)]
struct Summary {
  guard: u32,
  asleep: i64,
  by_minute: [u32; 60],
  best_minute: usize,
  best_asleep: u32,
}

pub fn solve() {
  let mut lines = read_lines();
  let mut by_guard_opt: Option<Vec<Summary>> = None;

  measure_and_print(|| {
    ::quickersort::sort_by(&mut lines, &|log1, log2| log1.moment.cmp(&log2.moment));
    by_guard_opt = Some(summarize_by_guard(&lines));
    println!("Common part");
  });

  let by_guard = by_guard_opt.unwrap();

  measure_and_print(|| {
    solve1(&by_guard);
  });

  measure_and_print(|| {
    solve2(&by_guard);
  });
}

fn solve1(by_guard: &[Summary]) {
  let max_asleep: Option<&Summary> = by_guard.iter().fold(None, |last, current| match last {
    None => Some(current),
    Some(candidate) => {
      if candidate.asleep > current.asleep {
        Some(candidate)
      } else {
        Some(current)
      }
    }
  });

  let result = max_asleep.unwrap();

  println!(
    "Result day 4 1/2 {}",
    result.guard * result.best_minute as u32
  );
}

fn solve2(by_guard: &[Summary]) {
  let most_asleep: Option<&Summary> = by_guard.iter().fold(None, |last, current| match last {
    None => Some(current),
    Some(candidate) => {
      if candidate.best_asleep > current.best_asleep {
        Some(candidate)
      } else {
        Some(current)
      }
    }
  });

  let result = most_asleep.unwrap();

  println!(
    "Result day 4 2/2 {}",
    result.guard * result.best_minute as u32
  );
}

fn summarize_by_guard(lines: &[Log]) -> Vec<Summary> {
  let mut asleep_time: HashMap<u32, Summary> = HashMap::new();
  let mut current_guard: Option<u32> = None;
  let mut last_moment: Option<LocalDateTime> = None;

  for log in lines {
    match log.guard {
      None => {
        let guard = current_guard.unwrap();
        let action = &log.action;
        let previous_moment = last_moment.unwrap();

        match action {
          Action::BeginsShift | Action::WakesUp => {
            let summary = asleep_time.entry(guard).or_insert(Summary {
              guard,
              asleep: 0,
              by_minute: [0; 60],
              best_minute: 0,
              best_asleep: 0,
            });

            summary.asleep +=
              (log.moment.to_instant().seconds() - previous_moment.to_instant().seconds()) / 60;

            let last_minute = log.moment.time().minute() as usize;
            let mut current_minute = previous_moment.time().minute() as usize;

            while current_minute != last_minute {
              summary.by_minute[current_minute] += 1;

              if summary.by_minute[current_minute] > summary.best_asleep {
                summary.best_asleep = summary.by_minute[current_minute];
                summary.best_minute = current_minute;
              }

              current_minute = (current_minute + 1) % 60;
            }
          }
          Action::FallsAsleep => {}
        }
      }
      Some(guard) => {
        current_guard = Some(guard);
      }
    };

    last_moment = Some(log.moment);
  }

  asleep_time.values().cloned().collect()
}

fn read_lines() -> Vec<Log> {
  let input_file = File::open("src/day_4/input.txt").expect("file not found");
  let parser =
    Regex::new("^\\[(\\d{4})-(\\d{2})-(\\d{2}) (\\d{2}):(\\d{2})\\] (?:Guard #(\\d+) )?(.*)$")
      .unwrap();

  BufReader::new(input_file)
    .lines()
    .map(|line| line.expect("error readding the line"))
    .map(|line| {
      let parsed = parser.captures(&line[..]).unwrap();

      let year: i64 = parse(&parsed, 1);
      let month: i8 = parse(&parsed, 2);
      let day: i8 = parse(&parsed, 3);

      let date = LocalDate::ymd(year, Month::from_one(month).unwrap(), day).unwrap();

      let hours: i8 = parse(&parsed, 4);
      let minutes: i8 = parse(&parsed, 5);

      let time = LocalTime::hm(hours, minutes).unwrap();
      let moment = LocalDateTime::new(date, time);

      match parsed.get(6) {
        None => Log {
          moment,
          guard: None,
          action: Action::parse(parsed.get(7).unwrap().as_str()).unwrap(),
        },
        Some(guard_capture) => Log {
          moment,
          guard: Some(guard_capture.as_str().parse().unwrap()),
          action: Action::BeginsShift,
        },
      }
    })
    .collect()
}

fn parse<T>(captures: &regex::Captures, index: usize) -> T
where
  T: core::str::FromStr,
  T::Err: std::fmt::Debug,
{
  captures.get(index).unwrap().as_str().parse().unwrap()
}
