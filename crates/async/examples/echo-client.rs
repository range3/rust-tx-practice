#![feature(await_macro, async_await, futures_api)]

use tokio::{await, net::TcpStream, prelude::*};

use std::{io, net::SocketAddr};

const MESSAGES: &[&str] = &["hello", "world", "one two three"];

fn main() {
  let addr = std::env::args()
    .nth(1)
    .unwrap_or("0.0.0.0:8080".to_string());
  let addr = addr.parse::<SocketAddr>().unwrap();

  tokio::run_async(
    async move {
      match await!(run_client(&addr)) {
        Ok(_) => println!("done."),
        Err(e) => println!("echo client failed; error = {:?}", e),
      }
    },
  );
}

async fn run_client(addr: &SocketAddr) -> io::Result<()> {
  let mut stream = await!(TcpStream::connect(addr))?;

  let mut buf = [0; 128];

  for msg in MESSAGES {
    println!("> write = {:?}", msg);

    await!(stream.write_all_async(msg.as_bytes()))?;

    await!(stream.read_exact_async(&mut buf[..msg.len()]))?;

    assert_eq!(&buf[..msg.len()], msg.as_bytes());
  }

  Ok(())
}
