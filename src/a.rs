pub mod b;

pub const A_PUB: &str = "A_PUB";
const A_PRIVATE: &str = "A_PRIVATE";

pub fn a_visit() {
    println!("\n>>> A_VISIT");

    // 1. 访问父模块 lib
    println!("lib_pub: {}", super::LIB_PUB);
    println!("{}", super::LIB_PRIVATE);     // ✅ 父对子无隐私

    // 2. 访问子模块 b
    println!("B_PUB_CRATE: {}", b::B_PUB_CRATE);
    println!("b_super: {}", b::B_PUB_SUPER);
    // println!("{}", b::B_PRIVATE);            // ❌ 无法访问子模块的私有成员

    // 3. 访问兄弟模块 x
    // 注意：x 在 lib 里是私有的，a 无法通过 super::x 访问它
    // 但可以通过根部重新导出的路径访问其公开成员
    // println!("x_pub_reexport: {}", crate::X_PUB); // ✅
    // println!("{}", crate::x::X_PUB);         // ❌ 无法穿透私有模块 x 的路径
}