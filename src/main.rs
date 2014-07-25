//	import our module dependencies into scope
use lib::experiments::{test_proc, test_ref, test_struct};

//	define the empty dependency module structure used to find modules on the filesystem
mod lib { pub mod experiments;}

fn main() {

	// test_proc();
	// test_ref();
	// test_struct();
	test_vec_func();
}


struct Httpo {
	middleware: Vec<fn(&mut Context)>,
	server: int
}

impl Httpo {

	fn new () -> Httpo {

		Httpo {middleware: Vec::new(), server: 10i}
	}

	fn utilise (mut self, mw: fn(&mut Context)) {
		self.middleware.push(mw);
	}
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

	let mut app = box Httpo::new();

	app.utilise(mid_1);
	app.utilise(mid_2);

	let mut ctx = Context {status: 200, body: String::new()};

	for f in app.middleware.iter() {
		let func = *f;
		func(&mut ctx);
	}

	println!("middleware: {}", ctx.body);

}
