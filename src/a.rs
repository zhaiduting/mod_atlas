pub mod b;

pub const A_PUB: &str = "A_PUB";
const A_PRIVATE: &str = "A_PRIVATE";

pub fn a_visit() {
    println!("\n>>> A_VISIT");

    // 1. 访问父模块 lib
    println!("lib_pub: {}", super::LIB_PUB);
    println!("{}", super::LIB_PRIVATE);     // ✅父辈隐私透明化，可访

    // 2. 访问子模块 b
    println!("B_PUB_CRATE: {}", b::B_PUB_CRATE);
    println!("b_super: {}", b::B_PUB_SUPER);
    // println!("{}", b::B_PRIVATE);            // ❌ 无法访问子模块的私有成员

    // 3. 访问兄弟模块 x
    // 注意：虽然 x 在 lib 里是私有的，但 a 可以通过 super::x 访问它（符合「父对子无隐私」的规则）
    // a 可以进一步访问 super::x::X_PUB
    // a 不可以进一步访问 super::x::PRIVATE（符合「兄弟之间有隐私」的规则）
    println!("crate::x::X_PUB: {}", super::x::X_PUB);         // 兄弟 pub 可访
    // println!("crate::x::X_PUB: {}", crate::x::X_PRIVATE);     // ❌ 兄弟隐私不可访
}