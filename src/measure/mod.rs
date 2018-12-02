use elapsed::measure_time;

pub fn measure_and_print<F: FnOnce()>(callback: F) {
  let (time, _sum) = measure_time(|| {
    callback();
  });

  println!(
    "Time {}s {}ms {}μs",
    time.seconds(),
    time.millis() - (time.seconds() * 1000),
    time.micros() - (time.seconds() * 1e6 as u64) - (time.millis() * 1e3 as u64)
  );
}
