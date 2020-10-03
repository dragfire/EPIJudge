use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod color;
mod thread_pool;
pub mod tree;
use color::Color;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_test_data_path(filename: &str) -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push("../test_data/");
    path.push(filename);
    Ok(path)
}

/// Read `tsv` test data in `test_data` directory
///
/// Ignore the first row that contains header information.
/// Use each column except the last column.
/// Second last column is the expected result for each test case.
/// Other columns are function arguments.
pub fn read_test_data(filename: &str) -> Result<Vec<Vec<String>>> {
    let path = get_test_data_path(filename)?;
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // skip headers row
    lines.next();

    lines
        .map(|line| {
            line.and_then(|line| Ok(line.split('\t').map(String::from).collect::<Vec<_>>()))
                .map_err(|e| e.into())
        })
        .collect()
}

/// Represent current Execution Status of a test case.
///
/// This is used to communicate with a spawned child thread and parent.
enum ExecStatus {
    Done,
    /// (num_failed, num_passed)
    Failed((i32, i32)),
}

/// Run all test cases after reading all test data for a specific problem.
///
/// All tests are executed in a thread.
/// Handles all Runtime, Timeout errors.
pub fn run_tests<F>(filename: &'static str, func: F)
where
    F: 'static + Fn(Vec<String>) -> Result<()> + Send + std::panic::RefUnwindSafe,
{
    let (sender, receiver) = mpsc::channel::<ExecStatus>();

    thread::spawn(move || {
        let test_data = read_test_data(filename).unwrap();
        let mut test_nbr = 1;
        let n = test_data.len();
        let mut num_passed = 0;
        let mut num_failed = 0;

        for data in test_data {
            let curr_test_then = Instant::now();
            let result = func(data);
            match result {
                Ok(_) => {
                    println!(
                        "Test {} {}/{} [{:^5}µs]",
                        Color::Green("PASSED").make(),
                        test_nbr,
                        n,
                        curr_test_then.elapsed().as_micros()
                    );
                    num_passed += 1;
                }
                Err(_) => {
                    println!(
                        "Test {} {}/{} [{:^5}µs]",
                        Color::Red("FAILED").make(),
                        test_nbr,
                        n,
                        curr_test_then.elapsed().as_micros()
                    );
                    num_failed += 1;
                }
            }
            test_nbr += 1;
        }

        if num_failed > 0 {
            sender
                .send(ExecStatus::Failed((num_failed, num_passed)))
                .unwrap();
        } else {
            sender.send(ExecStatus::Done).unwrap();
        }
    });

    // TODO make default timeout configurable
    match receiver.recv_timeout(Duration::from_millis(5000)) {
        Ok(status) => match status {
            ExecStatus::Failed((failed, passed)) => {
                println!(
                    "{}",
                    Color::Red(&format!(
                        "*** {}/{} Tests FAILED. Oops!***",
                        failed,
                        failed + passed
                    ))
                    .make()
                );
            }
            ExecStatus::Done => {
                println!(
                    "{} {}",
                    Color::Cyan("All tests PASSED.").make(),
                    Color::Yellow("Congratulations!").make()
                );
            }
        },
        Err(e) => {
            panic!("TimeLimitExceeded: {:?}", e);
        }
    }
}

#[macro_export]
macro_rules! try_assert {
    ($l:expr, $r:expr) => {
        if $l == $r {
            Ok(())
        } else {
            Err("LHS != RHS".into())
        }
    };
}
