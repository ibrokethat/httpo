
use lib::experiments::{test_proc, test_ref, test_struct};
mod lib {
	pub mod experiments;
}

fn main() {

	// test_proc();
	// test_ref();
	// test_struct();
	test_vec_func();
}


struct Context {
	status: int,
	body: String
}


fn mid_1 (ctx: &mut Context) {

	ctx.body.push_str("mid_1");
	// ctx
}


fn mid_2 (ctx: &mut Context) {

	ctx.body.push_str("mid_2");
	// ctx
}



fn test_vec_func () {

	// let mut v = Vec::new();

	// v.push(mid_1);
	// v.push(mid_2);

	let v = [mid_1, mid_2];

	let mut ctx = Context {status: 200, body: String::new()};

	for f in v.iter() {
		let func = *f;
		func(&mut ctx);
	}

	println!("middleware: {}", ctx.body);

}
