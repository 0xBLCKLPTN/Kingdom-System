
fn main() {
    tonic_build::compile_protos("proto/instance.proto").unwrap();
}
