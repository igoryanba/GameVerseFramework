fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Компилируем proto файлы
    tonic_build::compile_protos("proto/logging.proto")?;
    Ok(())
} 