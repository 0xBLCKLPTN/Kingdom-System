fn main() {
    tonic_build::compile_protos("proto/database.proto").unwrap();
}

