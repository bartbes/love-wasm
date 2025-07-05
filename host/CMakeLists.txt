cmake_minimum_required(VERSION 3.20)

project("love-wasm-host" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include(FetchContent)

FetchContent_Declare(
	wasmtime
	GIT_REPOSITORY https://github.com/bytecodealliance/wasmtime.git
	GIT_TAG 7d3e57e730924c4a484645554adcfed1c6905ebe
	SOURCE_SUBDIR crates/c-api
	CMAKE_ARGS -DBUILD_SHARED_LIBS=ON
)

FetchContent_Declare(
	love
	GIT_REPOSITORY https://github.com/love2d/love.git
	GIT_TAG 11.5
	PATCH_COMMAND patch -p1 --forward -r- < ${CMAKE_CURRENT_SOURCE_DIR}/love-visibility.patch || true
)

FetchContent_MakeAvailable(wasmtime love)

add_executable(host
	main.cpp
)
target_include_directories(host
	PRIVATE ${love_SOURCE_DIR}/src ${love_SOURCE_DIR}/src/modules ${LOVE_INCLUDE_DIRS})
target_link_libraries(host
	PRIVATE wasmtime-cpp liblove
)
