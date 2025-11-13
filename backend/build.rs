fn main() {
    println!("cargo:rerun-if-changed=../proto/containers.proto");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&["../proto/containers.proto"], &["../proto"])
        .expect("failed to compile proto definitions");
}
