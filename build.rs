#[allow(dead_code)]
fn build_proto() {}

fn main() {
    #[cfg(feature = "rebuild-proto")]
    fn build_proto() {
        // Change prost configuration in order to output files to /src
        let out = std::path::Path::new(&std::env::var("CARGO_PKG_REPOSITORY").unwrap()).join("src");
        let mut config = prost_build::Config::new();
        config.out_dir(out);

        // Directory for the proto files
        let directory = "protobuf";

        // Pick which proto files to build
        for entry in &["types.proto", "data.proto", "protocol.proto"] {
            config.compile_protos(&[entry], &[&directory]).unwrap();
        }
    }

    // This only runs if rebuild-proto feature is enabled
    build_proto();
}
