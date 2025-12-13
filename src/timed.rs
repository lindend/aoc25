use std::time::{Duration, Instant};

pub fn print_timespan(title: &str, time: Duration) {
    let millis = time.subsec_millis();
    let micros = time.subsec_micros() - millis * 1000;
    let nanos = time.subsec_nanos() - millis * 1000000 - micros * 1000;
    println!(
        "{title} took {}s {}ms {}us {}ns",
        time.as_secs(),
        millis,
        micros,
        nanos
    );
}
pub fn timed<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let start = Instant::now();

    let result = f();

    let end = Instant::now();
    
    print_timespan("Execution", end - start);

    result
}
