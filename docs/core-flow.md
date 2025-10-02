|Platform	|Cross-platform Desktop Application (Windows, macOS, Linux).|
|Primary Goal	|Demonstrate high-performance computation (C++) integrated safely with a modern, stable application layer (Rust).|
|UI Framework	|Use a framework that integrates well with C++, such as Qt (via C++) or a Rust-native framework like Tauri/EGUI that can still call C++ via Rust FFI. (We will proceed assuming the C++ side manages the UI, a common pattern for Qt.)|
|Core Flow	|UI (C++) → Safe Rust API → C++ Compute Kernel → Safe Rust Validation → UI (C++)|
