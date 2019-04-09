#![feature(duration_float)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

#[derive(Debug)]
struct Item {
  val: i32,
}

impl Item {
  fn new() -> Item {
    Item { val: 0 }
  }
}

fn main() {
  let mut handles = vec![];

  let mut items = vec![];
  for _ in 0..100 {
    items.push(Mutex::new(Item::new()));
  }
  let items = Arc::new(items);

  let nthreads = 10;
  let elapsed_time = time::Instant::now();

  for id in 0..nthreads {
    let items = Arc::clone(&items);
    let handle = thread::spawn(move || {
      let mut item1 = items[id].lock().unwrap();
      item1.val += id as i32 + 1;

      let mut item2 = items[id + 1].lock().unwrap();
      item2.val += id as i32 + 1;
    });
    handles.push(handle);
  }

  let elapsed_time = elapsed_time.elapsed();

  for handle in handles {
    handle.join().unwrap();
  }

  println!("{:?}", items);
  println!("Elapsed time(sec): {}", elapsed_time.as_secs_f64());
  println!("Throuput (trans/sec): {}", nthreads as f64 / elapsed_time.as_secs_f64());
}
