fn main() {
    tonic_build::compile_protos("proto/queue.proto").unwrap();
}