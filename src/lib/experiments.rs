pub fn test_proc () {

	for num in range(0u, 50) {

		let (chan, port) = channel();

		spawn(proc() {

			chan.send(num);

			println!("proc {0}", num);
		});

		println!("port {:s}", port.recv().to_string());

	}


}



	// let f = init_routes();
	// println!("init_routes {0}", f()[0]);

// pub fn init_routes() -> fn() -> &[int] {

// 	fn funcy() -> &[int] {
// 		[4i]
// 	}
// 	funcy

// }



pub fn test_struct () {

	struct Point {
		x: int,
		y: int
	}

	let pp = Point {x: 1, y: 10};

	fn by_struct (p: &Point) {
		println!("by struct: {}, {}", p.x, p.y);
	}

	by_struct(&pp);

}


pub fn test_ref () {

	let ii = 1i;

	fn by_ref (i: &int) {
		println!("by ref: {}", i);
	}

	by_ref(&ii);

}


// pub fn test_val () {

//     let mut ii:Box<int> = 1;

//     fn by_val(i: Box<int>) {
//         i = 2i;
//         println!("by ref: {}", i);
//     }

//     by_val(ii);

// }

