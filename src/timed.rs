use std::time::Instant;

pub fn timed<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let start = Instant::now();

    let result = f();

    let end = Instant::now();

    let execution_time = end - start;
    let millis = execution_time.subsec_millis();
    let micros = execution_time.subsec_micros() - millis * 1000;
    let nanos = execution_time.subsec_nanos() - millis * 1000000 - micros * 1000;
    println!(
        "Execution took {}s {}ms {}us {}ns",
        execution_time.as_secs(),
        millis,
        micros,
        nanos
    );

    result
}
