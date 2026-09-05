cmake_minimum_required(VERSION 3.24)
project(GameVerseNativeBootstrap VERSION 0.1.0 LANGUAGES CXX)

if(NOT WIN32 OR NOT CMAKE_SIZEOF_VOID_P EQUAL 8)
  message(FATAL_ERROR "GameVerse.NativeBootstrap requires 64-bit Windows")
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include(FetchContent)
FetchContent_Declare(
  minhook
  GIT_REPOSITORY https://github.com/TsudaKageyu/minhook.git
  GIT_TAG c3fcafdc10146beb5919319d0683e44e3c30d537
  GIT_SHALLOW FALSE
)
FetchContent_MakeAvailable(minhook)

add_library(gameverse_bootstrap_core STATIC
  src/compatibility.cpp
  src/fingerprint.cpp
  src/pe_image.cpp
  src/signature_scanner.cpp
  src/state_machine.cpp
  src/telemetry.cpp
)
target_include_directories(gameverse_bootstrap_core PUBLIC include)
target_compile_definitions(gameverse_bootstrap_core PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
target_link_libraries(gameverse_bootstrap_core PUBLIC bcrypt user32 version)

add_library(GameVerse.NativeBootstrap SHARED
  src/bootstrap.cpp
  src/dllmain.cpp
  src/pipe_client.cpp
)
set_target_properties(GameVerse.NativeBootstrap PROPERTIES PREFIX "" SUFFIX ".asi")
target_include_directories(GameVerse.NativeBootstrap PRIVATE include)
target_compile_definitions(GameVerse.NativeBootstrap PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
target_link_libraries(GameVerse.NativeBootstrap PRIVATE gameverse_bootstrap_core minhook)

add_executable(gameverse_native_bootstrap_tests tests/tests.cpp)
target_link_libraries(gameverse_native_bootstrap_tests PRIVATE gameverse_bootstrap_core)
target_compile_definitions(gameverse_native_bootstrap_tests PRIVATE
  GAMEVERSE_TEST_MANIFEST=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/enhanced-1.0.1158.13.json"
  GAMEVERSE_TEST_SIGNATURE=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/enhanced-1.0.1158.13.sig"
)
enable_testing()
add_test(NAME native_bootstrap_tests COMMAND gameverse_native_bootstrap_tests)

foreach(target gameverse_bootstrap_core GameVerse.NativeBootstrap gameverse_native_bootstrap_tests)
  if(MSVC)
    target_compile_options(${target} PRIVATE /W4 /WX /permissive- /EHsc /utf-8)
  endif()
endforeach()

install(TARGETS GameVerse.NativeBootstrap RUNTIME DESTINATION . LIBRARY DESTINATION .)
install(FILES
  compatibility/enhanced-1.0.1158.13.json
  compatibility/enhanced-1.0.1158.13.sig
  DESTINATION .
)
