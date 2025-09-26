fn main() {
    // 设置 TEST_FOO 环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 设置 cfg 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}