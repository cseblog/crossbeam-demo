use crossbeam::channel::*;
use std::thread;
use std::time::Instant;

fn main() {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;
    let (s, r) = bounded(buf_size);
    let s2 = s.clone();

    let start_time = Instant::now();
    // Producer 1
    let t1 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            s.send(1).unwrap();
        }
    });

    // Producer 2
    let t2 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            s2.send(1).unwrap();
        }
    });

    //Consumer
    let mut sum = 0;
    for msg in r {
        let tmp = msg;
        sum += tmp;
    }

    let _ = t1.join();
    let _ = t2.join();

    let d = Instant::now().duration_since(start_time);
    let delta = d.as_millis();
    println!("Sum: {}, processed  time: {}", sum, delta);
}
