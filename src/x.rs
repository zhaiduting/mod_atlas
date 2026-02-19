pub const X_PUB: &str = "X_PUB";
const X_PRIVATE: &str = "X_PRIVATE";

pub fn x_visit() {
    println!("\n>>> X_VISIT in private mod of lib.rs");

    // 1. 访问父模块 lib
    inspect!(   super::LIB_PUB);    // ✅
    inspect!( super::LIB_PRIVATE);    // ✅

    // 2. 访问兄弟模块 a
    inspect!(super::a::A_PUB);     // ✅
    inspect!(  super  ::a::b::B_PUB   );     // ✅
}