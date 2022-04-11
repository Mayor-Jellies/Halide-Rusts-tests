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

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).display()
    );
}
/*
system(c lib)
std::process::comand(path, argv)
--help

g++ -O3 -std=c++17 -I /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/include/ -I /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/  -Wall -Werror -Wno-unused-function -Wcast-qual -Wignored-qualifiers -Wno-comment -Wsign-compare -Wno-unknown-warning-option -Wno-psabi -g iir_blur_generator.cpp /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/GenGen.cpp -o bin/host/iir_blur.generator -Wl,-rpath,/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/ -L /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/ -lHalide -ldl -lpthread -lz -Wl,--no-as-needed -lautoschedule_mullapudi2016 -Wl,--as-needed
bin/host/iir_blur.generator -g iir_blur -f iir_blur -o bin/host target=host-no_runtime auto_schedule=false
running: "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-I" "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/include/" "-I" "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/" "-Wall" "-Wextra" "-Wall" "-Wno-unused-function" "-Wcast-qual" "-Wignored-qualifiers" "-o" "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/apps/iir_blur/iir_blur_generator.cpp /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/GenGen.o" "-c" "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/apps/iir_blur/iir_blur_generator.cpp /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/GenGen.cpp"

*/
