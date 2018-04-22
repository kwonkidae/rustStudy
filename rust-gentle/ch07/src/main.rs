use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time;

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
}
