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

set(ZYDIS_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(ZYDIS_BUILD_TOOLS OFF CACHE BOOL "" FORCE)
set(ZYDIS_BUILD_TESTS OFF CACHE BOOL "" FORCE)
FetchContent_Declare(
  zydis
  GIT_REPOSITORY https://github.com/zyantific/zydis.git
  GIT_TAG a2278f1d254e492f6a6b39f6cb5d1f5d515659dc
  GIT_SHALLOW FALSE
)
FetchContent_MakeAvailable(zydis)

add_library(gameverse_bootstrap_core STATIC
  src/compatibility.cpp
  src/fingerprint.cpp
  src/pe_image.cpp
  src/pipe_client.cpp
  src/signature_scanner.cpp
  src/state_machine.cpp
  src/telemetry.cpp
)
target_include_directories(gameverse_bootstrap_core PUBLIC include)
target_compile_definitions(gameverse_bootstrap_core PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
target_link_libraries(gameverse_bootstrap_core PUBLIC bcrypt user32 version Zydis)

add_library(GameVerse.NativeBootstrap SHARED
  src/bootstrap.cpp
  src/dllmain.cpp
  src/observe_hooks.cpp
)
set_target_properties(GameVerse.NativeBootstrap PROPERTIES PREFIX "" SUFFIX ".asi")
target_include_directories(GameVerse.NativeBootstrap PRIVATE include)
target_compile_definitions(GameVerse.NativeBootstrap PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
target_link_libraries(GameVerse.NativeBootstrap PRIVATE gameverse_bootstrap_core minhook)

add_executable(gameverse_native_bootstrap_tests tests/tests.cpp src/observe_hooks.cpp)
target_link_libraries(gameverse_native_bootstrap_tests PRIVATE gameverse_bootstrap_core minhook)
target_compile_definitions(gameverse_native_bootstrap_tests PRIVATE
  GAMEVERSE_TEST_MANIFEST=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/enhanced-1.0.1158.13.json"
  GAMEVERSE_TEST_SIGNATURE=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/enhanced-1.0.1158.13.sig"
  GAMEVERSE_TEST_CANDIDATES=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/telemetry-candidates-v1.json"
  GAMEVERSE_TEST_CANDIDATE_SIGNATURE=L"${CMAKE_CURRENT_SOURCE_DIR}/compatibility/telemetry-candidates-v1.sig"
)
enable_testing()
add_test(NAME native_bootstrap_tests COMMAND gameverse_native_bootstrap_tests)

add_executable(gameverse-native-inspector tools/native-inspector.cpp)
target_link_libraries(gameverse-native-inspector PRIVATE gameverse_bootstrap_core Zydis)
target_compile_definitions(gameverse-native-inspector PRIVATE UNICODE _UNICODE WIN32_LEAN_AND_MEAN NOMINMAX)
add_test(NAME native_inspector_self_scan COMMAND gameverse-native-inspector
  --image $<TARGET_FILE:gameverse-native-inspector> --string schema_version)
add_test(NAME native_inspector_reference_scan COMMAND gameverse-native-inspector
  --image $<TARGET_FILE:gameverse-native-inspector> --reference-page-rva 0)

foreach(target gameverse_bootstrap_core GameVerse.NativeBootstrap gameverse_native_bootstrap_tests gameverse-native-inspector)
  if(MSVC)
    target_compile_options(${target} PRIVATE /W4 /WX /permissive- /EHsc /utf-8)
  endif()
endforeach()

install(TARGETS GameVerse.NativeBootstrap gameverse-native-inspector
  RUNTIME DESTINATION . LIBRARY DESTINATION .)
install(FILES
  compatibility/enhanced-1.0.1158.13.json
  compatibility/enhanced-1.0.1158.13.sig
  compatibility/telemetry-candidates-v1.json
  compatibility/telemetry-candidates-v1.sig
  DESTINATION .
)
install(FILES THIRD_PARTY_MINHOOK.txt THIRD_PARTY_ZYDIS.txt DESTINATION licenses)
