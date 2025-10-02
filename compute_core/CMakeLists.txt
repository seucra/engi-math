# compute_core/CMakeLists.txt

cmake_minimum_required(VERSION 3.10)
project(MathComputeCore CXX)

# Find the Eigen3 library (installed via pacman)
find_package(Eigen3 REQUIRED)

# Define our static library
add_library(math_compute_core STATIC math_core.cpp)

# Publicly link the Eigen targets. When CMake finds a package, it often
# creates an imported target (e.g., Eigen3::Eigen). Linking the target
# automatically handles the include directories and defines.
target_link_libraries(math_compute_core PUBLIC Eigen3::Eigen)

# Also ensure our local header directory is included for 'math_core.h'
target_include_directories(math_compute_core PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

# Set C++ standard
target_compile_features(math_compute_core PUBLIC cxx_std_17)

# Add the install step required by the Rust 'cmake' crate build method.
install(TARGETS math_compute_core
    DESTINATION lib
)

