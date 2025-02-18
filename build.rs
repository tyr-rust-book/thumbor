fn main() {
    println!("cargo:rerun-if-changed=abi.proto");
    println!("cargo:rerun-if-changed=build.rs");
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
