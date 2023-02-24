use std::thread;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tokio::sync::Mutex;
use tokio::runtime::Runtime;

fn main() {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	let running = Arc::new(Mutex::new(true));
	let running_1 = running.clone();
	let handler = thread::spawn(move || {
		if let Ok(rt)  = Runtime::new() {
			rt.block_on(async move {
				tokio::spawn(async {
					for i in 1..50 {
						log::info!("Print 2 in runtime {}", i);
					}
				});

				tokio::spawn(async {
					for i in 1..10 {
						log::info!("Print 3 in runtime {}", i);
					}
				});

				tokio::spawn(async {
					for i in 1..10 {
						log::info!("Print 4 in runtime {}", i);
					}
				});

				let running_2 = running_1.clone();
				tokio::spawn(async move {
					for i in 1..10 {
						log::info!("Print 5 in runtime {}", i);
						sleep(Duration::from_millis(200)).await;
					}
					let mut lock = running_2.lock().await;
					*lock = false;
				});

				for i in 1..10 {
					log::info!("Print 1 in runtime {}", i);
				}

				loop {
					sleep(Duration::from_secs(1)).await;
					let lock = running_1.lock().await;
					if !(*lock) {
						break;
					}
				}
			});
		}
	});

	for i in 1..20 {
		log::info!("Print in main thread {}", i);
		thread::sleep(std::time::Duration::from_secs(1));
	}

	handler.join().unwrap();
	log::info!("Finished!");
}
