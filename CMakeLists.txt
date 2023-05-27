cmake_minimum_required(VERSION 3.26)
project(RayDeezNutz)

set(CMAKE_CXX_STANDARD 17)

# Set the path to the Raylib library
set(RAYLIB_DIR "/usr/local/")

# Set the path to the Raylib includes
set(RAYLIB_INCLUDE_DIR "${RAYLIB_DIR}/include")

# Set the path to the Raylib library files
set(RAYLIB_LIBRARY_DIR "${RAYLIB_DIR}/lib")

# Set the Raylib libraries you want to link against
set(RAYLIB_LIBRARIES
    "${RAYLIB_LIBRARY_DIR}/libraylib.so.450"
    # Add any additional Raylib libraries you need here
)

# Add the Raylib include directory
include_directories(${RAYLIB_INCLUDE_DIR})


# Add your source files
set(SOURCES
    src/main.cpp
    include/app.hpp
    # Add any additional source files here
)

# Create the executable and link against the Raylib libraries
add_executable(RayDeezNuts ${SOURCES})
target_link_libraries(RayDeezNuts ${RAYLIB_LIBRARIES})

