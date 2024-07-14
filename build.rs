fn main() {
    tonic_build::compile_protos("proto/arikedbpbuff.proto").unwrap();
}
