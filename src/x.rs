pub const X_PUB: &str = "X_PUB";
const X_PRIVATE: &str = "X_PRIVATE";

pub fn x_visit() {
    println!("\n>>> X_VISIT (Hidden)");

    // 1. 访问父模块 lib
    println!("lib_pub: {}", super::LIB_PUB);    // ✅

    // 2. 访问兄弟模块 a
    println!("a_pub: {}", crate::a::A_PUB);     // ✅
}