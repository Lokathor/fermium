
cargo clean

cargo +stable-x86_64-pc-windows-msvc build --features="dynamic_link, use_bindgen_bin"

cargo +stable-i686-pc-windows-msvc build --features="dynamic_link, use_bindgen_bin"
