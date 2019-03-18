extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/proto",
        input: &["src/descriptor.proto"],
        includes: &[],
        customize: protoc_rust::Customize::default(),
    })
    .expect("protoc");
}

