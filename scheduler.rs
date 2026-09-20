pub fn run_scheduler() {
    loop {
        println!("Checking app expiry...");
        std::thread::sleep(std::time::Duration::from_secs(60 * 60 * 6));
    }
}