//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    println!("cargo:rustc-cfg=pass");
}
// fn main() {
//     // 处理 tests7: 设置环境变量 TEST_FOO=时间戳
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//     println!("cargo:rustc-env=TEST_FOO={}", timestamp); // ✅ 正确设置环境变量

//     // 处理 tests8: 启用 `pass` 条件编译
//     println!("cargo:rustc-cfg=pass"); // ✅ 启用编译条件
// }