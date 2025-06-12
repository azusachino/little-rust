fn main() -> std::io::Result<()> {
    // tonic_build::configure()
    // .type_attribute("routeguide.Point", "#[derive(Hash)]")
    // .compile(&["src/proto/routeguide.proto"], &["src"])
    // .unwrap();

    // prost_build::Config::new()
    // .out_dir("src/pb")
    // .compile_protos(&["src/proto/abi.proto", "src/proto/hello.proto"], &["src"])?;
    // prost_build::Config::new()
    //     .out_dir("src/excercises/pb")
    //     .compile_protos(&["src/proto/abi.proto", "src/proto/hello.proto"], &["src"])?;

    Ok(())
}
