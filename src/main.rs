use tokio::time::{sleep, Duration};

use daemonize::Daemonize;

fn test_run() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            loop {
                sleep(Duration::from_secs(10)).await;
            }
        })
}

fn main() {
    // add args parser to take `-d` as daemon and decode `-c config` config.json
    let daemonize = Daemonize::new();
    match daemonize.start() {
        Ok(_) => println!("success"),
        Err(e) => println!("Error: {}", e),
    }

    println!("start ok"); // won't see

    test_run();
}
