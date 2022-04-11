//extern crate bindgen;generatorStuffgeneratorStuff

use std::env;
use std::path::Path;
use std::process::Command;
use std::io;
use std::io::prelude::*;

fn main() {

    //let Halide_path = Path::new("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide");
	//env::set_var("RUST_BACKTRACE", "1");


    let output = Command::new("gcc")
		.args(["-O3","-std=c++17",
			"-I","/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/include/",
			"-I", "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/",
			"-g", "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/iir_blur_generator.cpp","/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/GenGen.cpp",
			"-o", "iir_blur_command.gen",
			"-Wl,-rpath,/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/",
			"-L","/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/",
			"-lHalide","-lstdc++","-ldl","-lpthread","-lz","-Wl,--no-as-needed","-lautoschedule_mullapudi2016","-Wl,--as-needed"])
		.output()
		.expect("Building the gererator failed");

	println!("status: {}", output.status);
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();
	assert!(output.status.success());

	let gen = Command::new("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/iir_blur_command.gen")
		.args(["-g","iir_blur","-f", "iir_blur","-o", "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/","target=host-no_runtime","target=host-no_runtime"])
		.output()
		.expect("running the gererator failed");
	println!("status: {}", gen.status);
	io::stdout().write_all(&gen.stdout).unwrap();
	io::stderr().write_all(&gen.stderr).unwrap();
	assert!(gen.status.success());

	let run = Command::new("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/iir_blur_command.gen")
		.args(["-r","runtime","-o","/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/","target=host"])
		.output()
		.expect("running the gererator failed");
	println!("status: {}", run.status);
	io::stdout().write_all(&run.stdout).unwrap();
	io::stderr().write_all(&run.stderr).unwrap();
	assert!(run.status.success());

	let rename_iir = Command::new("mv")
		.args(["iir_blur.a", "libiir_blur.a"])
		.output()
		.expect("renaming the .h failed");
	println!("status: {}", rename_iir.status);
	io::stdout().write_all(&rename_iir.stdout).unwrap();
	io::stderr().write_all(&rename_iir.stderr).unwrap();
	assert!(rename_iir.status.success());

	let rename_runtime = Command::new("mv")
		.args(["runtime.a", "libruntime.a"])
		.output()
		.expect("renaming the .h failed");
	println!("status: {}", rename_runtime.status);
	io::stdout().write_all(&rename_runtime.stdout).unwrap();
	io::stderr().write_all(&rename_runtime.stderr).unwrap();
	assert!(rename_runtime.status.success());


    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).display()
    );
}
