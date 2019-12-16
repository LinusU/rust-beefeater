use std::sync::Arc;
use std::thread;
use std::time::Duration;

use beefeater::Beefeater;
use beefeater::ops::*;

fn main() {
    let number = Arc::new(Beefeater::new(10_000));

    let add = {
        let number = Arc::clone(&number);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1));
                number.add_assign(1);
            }
        })
    };

    let sub = {
        let number = Arc::clone(&number);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1));
                number.sub_assign(1);
            }
        })
    };

    let print = {
        let number = Arc::clone(&number);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                println!("{}", number.load());
            }
        })
    };

    add.join().unwrap();
    sub.join().unwrap();
    print.join().unwrap();
}
