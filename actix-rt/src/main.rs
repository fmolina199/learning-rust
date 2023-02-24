use actix_rt::System;
use std::thread;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tokio::sync::Mutex;

fn main() {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	let running = Arc::new(Mutex::new(true));
	let running_1 = running.clone();
	let handler = thread::spawn(|| {
		System::with_tokio_rt(|| {
			// build system with a multi-thread tokio runtime.
			tokio::runtime::Builder::new_multi_thread()
				.worker_threads(4)
				.enable_all()
				.build()
				.unwrap()
		})
		.block_on(async move {
			System::current().arbiter().spawn(async {
				for i in 1..10 {
					log::info!("Print 2 in runtime {}", i);
				}
			});

			System::current().arbiter().spawn(async {
				for i in 1..10 {
					log::info!("Print 3 in runtime {}", i);
				}
			});

			System::current().arbiter().spawn(async {
				for i in 1..10 {
					log::info!("Print 4 in runtime {}", i);
				}
			});

			let running_2 = running_1.clone();
			System::current().arbiter().spawn(async move {
				for i in 1..10 {
					log::info!("Print 5 in runtime {}", i);
				}
				let mut lock = running_2.lock().await;
				*lock = true;
			});

			for i in 1..10 {
				log::info!("Print 1 in runtime {}", i);
			}

			loop {
				sleep(Duration::from_secs(1)).await;
				let lock = running_1.lock().await;
				if *lock {
					break;
				}
			}
		});
	});

	for i in 1..20 {
		log::info!("Print in main thread {}", i);
		thread::sleep(std::time::Duration::from_secs(1));
	}

	handler.join().unwrap();
	log::info!("Finished!");
}
