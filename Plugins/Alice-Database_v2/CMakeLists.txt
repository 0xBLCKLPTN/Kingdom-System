cmake_minimum_required(VERSION 3.10)
project(AliceDatabase)

# Укажите целевую платформу
set(TARGET_PLATFORM "x86_64-pc-windows-gnu" CACHE STRING "Target platform")

# Явно укажите путь к Rust, если find_program не работает
set(CARGO_PATH "cargo")  # Замените на фактический путь к rustc

# Определите команды сборки Rust
add_custom_target(RustBuild ALL
    COMMAND ${CARGO_PATH} build --release --target=${TARGET_PLATFORM}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Rust project for ${TARGET_PLATFORM}"
)

add_custom_target(RustClean
    COMMAND ${CARGO_PATH} clean
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Cleaning Rust project"
)
