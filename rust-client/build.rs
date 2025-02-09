fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/rpx")
        .compile_protos(&["../proto/device-client.proto"], &["../proto"])?;
    Ok(())
}
