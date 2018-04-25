use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::process::Command;
use std::net::*;
use std::io::prelude::*;
use std::io;
extern crate pipeliner;
use pipeliner::Pipeline;

fn main() {
	let answer = Cell::new(42);
	assert_eq!(answer.get(), 42);

	answer.set(77);

	assert_eq!(answer.get(), 77);

	let greeting = RefCell::new("hello".to_string());

	assert_eq!(*greeting.borrow(), "hello");
	assert_eq!(greeting.borrow().len(), 5);

	*greeting.borrow_mut() = "hola".to_string();

	assert_eq!(*greeting.borrow(), "hola");

	// let mut gr = greeting.borrow_mut();
	// *gr = "hola".to_string();

	// assert_eq!(*greeting.borrow(), "hola");

	let s = "hello dolly".to_string();
	let rs1 = Rc::new(s);
	let rs2 = rs1.clone();

	println!("len {}, {}", rs1.len(), rs2.len());

	thread::spawn(|| println!("hello"));
	thread::spawn(|| println!("dolly"));

	println!("so fine");

	thread::sleep(time::Duration::from_millis(100));

	let t = thread::spawn(|| {
		println!("hello");
		// panic!("I give up!");
	});
	println!("wait: {:?}", t.join());

	let mut threads = Vec::new();

	for i in 0..5 {
		let t = thread::spawn(move || {
			println!("hello {}", i);
		});
		threads.push(t);
	}

	for t in threads {
		t.join().expect("thread failed");
	}

	{
		let nthreads = 5;
		let mut threads = Vec::new();
		let barrier = Arc::new(Barrier::new(nthreads));

		for i in 0..nthreads {
			let barrier = barrier.clone();
			let t = thread::spawn(move || {
				println!("before wait {}", i);
				barrier.wait();
				println!("after wait {}", i);
			});
			threads.push(t);
		}

		for t in threads {
			t.join().unwrap();
		}
	}

	{
		let answer = Arc::new(Mutex::new(42));

		let answer_ref = answer.clone();
		let t = thread::spawn(move || {
			let mut answer = answer_ref.lock().unwrap();
			*answer = 55;
		});

		t.join().unwrap();

		let ar = answer.lock().unwrap();
		assert_eq!(*ar, 55);
	}

	{
		for result in (0..10).with_threads(4).map(move |x| x + 1) {
			println!("result: {}", result);
		}
	}
	{
		// let addresses: Vec<_> = (1..10).map(|n| format!("ping -c1 192.168.0.{}", n)).collect();
		// let n = addresses.len();

		// for result in addresses.with_threads(n).map(|s| shell(&s)) {
		// 	if result.1 {
		// 		println!("got: {}", result.0);
		// 	} else {
		// 		println!("error");
		// 	}
		// }

		{
			for res in "google.com:80".to_socket_addrs().expect("bad") {
				println!("got {:?}", res);
			}

			let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0", n)).collect();
			let n = addresses.len();

			for result in addresses.with_threads(n).map(|s| s.to_socket_addrs().unwrap().next().unwrap()) {
				println!("got: {:?}", result);
			}
		}
	}

	{
		thread::spawn(|| {
			let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

			for connection in listener.incoming() {
				match connection {
					Ok(stream) => {
						if let Err(e) = handle_connection(stream) {
							println!("error {:?}", e);
						}
						// let mut text = String::new();
						// stream.read_to_string(&mut text).expect("read failed");
						// println!("got '{}'", text);

					}
					Err(e) => { println! ("connection failed {}", e); }
				}
			}
		}).join();
	}
}

fn shell(cmd: &str) -> (String,bool) {
	let cmd = format!("{} 2>&1",cmd);
	let output = Command::new("/bin/sh")
		.arg("-c")
		.arg(&cmd)
		.output()
		.expect("no shell?");
	(
		String::from_utf8_lossy(&output.stdout).trim_right().to_owned(),
		output.status.success()
	)
}

fn handle_connection(stream: TcpStream) -> io::Result<()>{

	while true {
		let mut ostream = stream.try_clone()?;
		let mut rdr = io::BufReader::new(ostream);
		let mut text = String::new();
		println!("start !");
		rdr.read_line(&mut text)?;
		println!("got '{}'", text.trim_right());
		println!("end !");
	}

	Ok(())
}
