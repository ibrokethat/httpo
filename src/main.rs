
use lib::experiments::{test_proc, test_ref, test_struct};
mod lib {
	pub mod experiments;
}

fn main() {


	test_proc();
	test_ref();
	test_struct();

}


struct Context {
	status: int,
	body: String
}


fn mid_1 <'a>(ctx: &'a mut Context) {

	ctx.body.push_str("mid_1");
	// ctx
}


fn mid_2 <'a>(ctx: &'a mut Context) {

	ctx.body.push_str("mid_2");
	// ctx
}



fn test_vec_func () {

	let mut v = Vec::new();

	v.push(mid_1);
	v.push(mid_2);

	let mut ctx = Context {status: 200, body: String::new()};

	for f in v.iter() {
		let func = *f;
		func(&ctx);
	}

	println!("middleware: {}", ctx.body);

}
