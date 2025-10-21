use std::time::Instant;
use poly::parser::parse_variable_declaration;

fn main() {
    let input = "new myVar as int: 10;";
    let iterations = 100_000;
    let mut total_duration = 0;

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = parse_variable_declaration(input);
        let duration = start.elapsed();
        total_duration += duration.as_nanos();
    }

    let avg_duration = total_duration / iterations;
    println!("Average parsing time: {} nanoseconds", avg_duration);
}
