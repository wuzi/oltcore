use std::thread::sleep;
use std::time::{Duration, Instant};

use oltcore::ConnectionManager;
use r2d2::Pool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "127.0.0.1";
    let port: u16 = 2222;
    let username = "";
    let password = "";

    let pool = Pool::builder()
        .max_size(2)
        .idle_timeout(Some(Duration::from_secs(300)))
        .build(ConnectionManager::new(host, port, username, password))?;
    let test_duration = Duration::from_secs(60 * 60);
    let interval = Duration::from_secs(600);
    let burst_size = 3;

    let start = Instant::now();
    let mut iteration = 0u64;

    while start.elapsed() < test_duration {
        iteration += 1;
        println!("Iteration {iteration} start");

        for call_index in 0..burst_size {
            let call_start = Instant::now();
            let mut conn = pool.get()?;

            let terminals = conn.display_ont_autofind_all()?;
            println!(
                "Iteration {iteration} call {}: {} unmanaged ONTs in {:?}",
                call_index + 1,
                terminals.len(),
                call_start.elapsed()
            );
            for (i, ont) in terminals.iter().enumerate() {
                println!("  {}. {}", i + 1, ont.ont_sn_readable);
            }
        }

        let elapsed = start.elapsed();
        println!("Iteration {iteration} complete after {elapsed:?}");

        if elapsed < test_duration {
            sleep(interval);
        }
    }

    Ok(())
}
