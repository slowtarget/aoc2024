/// Measure the execution time of a block and print the result with its name.
#[macro_export]
macro_rules! measure_time {
    ($func:expr) => {{
        let start = Instant::now();
        let result = $func;
        let duration = start.elapsed();
        println!(
            "Function '{}' executed in {:?}",
            stringify!($func),
            duration
        );
        result
    }};
}