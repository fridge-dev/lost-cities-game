use tonic_build;
use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure()
        .out_dir("./src/")
        .compile(
            &["./lost_cities_wire.proto"],
            &["./"],
        )
}
