fn main() {
    tonic_build::compile_gateway_protos("proto/method_options.proto").unwrap();
}
