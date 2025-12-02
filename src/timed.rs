use std::time::{SystemTime, UNIX_EPOCH};

pub fn timed<F, T>(f: F) -> T where F: Fn() -> T {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    
    let result = f();
    
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    
    let execution_time = (end - start).as_millis();
    println!("Execution took {execution_time}ms");
    
    result
}