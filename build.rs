//extern crate bindgen;generatorStuffgeneratorStuff

use std::env;
use std::path::Path;
use std::process::Command;
use std::io;
use std::io::prelude::*;

fn main() {

    //let Halide_path = Path::new("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide");

    let output = Command::new("g++")
        .arg("-O3")
		.arg("-std=c++17")
		.arg("-I /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/include/")
		.arg("-I /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/")
		.arg("-g /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/iir_blur_generator.cpp /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools/GenGen.cpp")
		.arg("-o iir_blur_command.generator")
		.arg("-Wl,-rpath,/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/")
		.arg("-L /home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/")
		.arg("-lHalide")
		.arg("-ldl")
		.arg("-lpthread")
		.arg("-lz")
		.arg("-lautoschedule_mullapudi2016")
		.output()
		.expect("Building the gererator failed");

	println!("status: {}", output.status);
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();

	assert!(output.status.success());

/*
    cc::Build::new()
        .cpp(true)
        .cargo_metadata(false)
        //.cpp_link_stdlib("c++")
        .flag("-Wno-all")
        .debug(true)
        .flag("-Wno-unused-parameter")
        .include("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/include")
        .include("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/tools")
        .object("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/distrib/lib/libHalide.so")
        //.object("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/apps/iir_blur/iir_blur_generator.o")
        .flag("-lHalide")
        .flag("-lpthread")
        .flag("-ldl")
        .file()
        .file("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide/apps/iir_blur/iir_blur_generator.cpp")
        .out_dir("/home/rootbutcher2/CLionProjects/Halide-Rusts-tests")
        .compile("iir_blur_test.generator");
*/
    //RUSTFLAGS="-Z sanitizer=address";
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
