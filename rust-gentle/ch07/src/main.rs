use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::mpsc;
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

	let name = "dolly".to_string();
	// move가 없으면 컴파일 에러
	let t = thread::spawn(move || {
		println!("hello {}", name);
	});
	println!("wait {:?}", t.join());

	{
		let name = "dolly";
		let t1 = thread::spawn(move || {
			println!("hello {}", name);
		});

		let t2 = thread::spawn(move || {
			println!("goodbye{} ", name);
		});

		println!("{:?}", t1.join());
		println!("{:?}", t2.join());
	}

	{
		let mut threads = Vec::new();
		let name = Arc::new(MyString::new("dolly"));

		for i in 0..5 {
			let tname = name.clone();
			let t = thread::spawn(move || {
				println!("hello {} count {}", tname.0, i);
			});
			threads.push(t);
		}

		for t in threads {
			t.join().expect("thread failed");
		}
	}

	{
		let nthreads = 5;
		let (tx, rx) = mpsc::channel();

		for i in 0..nthreads {
			let tx = tx.clone();
			thread::spawn(move || {
				let response = format!("hello {}", i);
				tx.send(response).unwrap();
			});
		}

		for _ in 0..nthreads {
			println!("got {:?}", rx.recv());
		}
	}

	{
<<<<<<< HEAD
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
=======
		// block send operation 
		let (tx, rx) = mpsc::sync_channel(4);

		let t1 = thread::spawn(move || {
			for i in 0..5 {
				tx.send(i).unwrap();
>>>>>>> e57de96154cd0c4f070feab716c65db981f60d69
			}
		});

		// for _ in 0..5 {
		// 	let res = rx.recv().unwrap();
		// 	println!("{}", res);
		// }
		t1.join().unwrap();
	}
<<<<<<< HEAD

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
=======
>>>>>>> e57de96154cd0c4f070feab716c65db981f60d69

}

<<<<<<< HEAD
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
=======
struct MyString(String);

impl MyString {
	fn new(s: &str) -> MyString {
		MyString(s.to_owned())
	}
}
>>>>>>> e57de96154cd0c4f070feab716c65db981f60d69
