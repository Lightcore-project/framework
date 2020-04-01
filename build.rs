extern crate protoc_rust;
use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protocol",
        input: &["protos/transaction.proto"],
        includes: &["protos"],
        customize: Customize {
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            serde_derive: Some(true),
            ..Default::default()
        },
    })
    .expect("protoc");
}
