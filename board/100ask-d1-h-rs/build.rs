fn main() {
    println!("cargo:rustc-link-search=/work/rustsbi/SyterKit/build/src/drivers/chips/sun20iw1/CMakeFiles/chip_drivers-obj.dir/");
    println!("cargo:rustc-link-arg=sys-dram.c.o");
    println!("cargo:rustc-link-arg=-Tallwinner-rt.ld");
}
