fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/proto") // Генерируем код в директорию src/proto
        .compile(
            &["proto/inventory.proto"],
            &["proto"], // Путь к директории, где находятся proto файлы
        )?;
    Ok(())
} 