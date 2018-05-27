use postgres::{Connection, TlsMode};
use std::thread;
use std::time::Duration;
use postgres_monitoring::db_wrapper::DbWrapper;

mod db_wrapper;



pub fn start_monitoring(postgres_url: String) {
    println!("postgres url: {}", postgres_url);

    let result = DbWrapper::new(postgres_url);

    match &result {
        &Ok(ref _result) => println!("Connected"),
        &Err(ref _error) => println!("Connection error: {}", _error),
    }

    let db = result.ok();

    loop {
        thread::sleep(Duration::from_secs(1));

        println!("next ");
    }
}
