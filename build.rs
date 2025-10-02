// build.rs
use std::path::PathBuf;

fn main() {
    // Determine the absolute path to the project root
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let compute_core_dir = project_root.join("compute_core");
    
    // 1. Compile the C++ library using CMake
    let dst = cmake::Config::new(&compute_core_dir)
        .build();

    // 2. Tell Cargo where to find and link our static C++ library
    // This links libmath_compute_core.a, which contains our compiled C++ code.
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display()); 
    println!("cargo:rustc-link-lib=static=math_compute_core");
    
    // 3. Link the C++ standard library dynamically (REQUIRED for any C++ FFI)
    // CRITICAL: We explicitly DO NOT link -leigen3, as that file doesn't exist.
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // We explicitly re-add the standard linker search paths just in case.
    println!("cargo:rustc-link-search=native=/usr/lib/");
    println!("cargo:rustc-link-search=native=/usr/lib64/"); 
    
    // 4. Compile the C++/Rust bridge code using cxx-build
    cxx_build::bridge("src/lib.rs")
        // Use the absolute path for the C++ implementation file
        .file(compute_core_dir.join("math_core.cpp")) 
        .flag_if_supported("-std=c++17")
        
        // Include the core directory for math_core.h (relative path fix)
        .include(&compute_core_dir) 
        
        // Explicitly add the system Eigen header path for the cxx-build compiler
        .include("/usr/include/eigen3") 
        
        .compile("engi_math_bridge"); 

    // 5. Rerun detection
    println!("cargo:rerun-if-changed={}", compute_core_dir.join("CMakeLists.txt").display());
    println!("cargo:rerun-if-changed={}", compute_core_dir.join("math_core.cpp").display());
    println!("cargo:rerun-if-changed={}", compute_core_dir.join("math_core.h").display());
}

