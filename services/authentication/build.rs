fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Компилируем proto файлы для gRPC
    tonic_build::compile_protos("proto/auth.proto")?;
    Ok(())
} 