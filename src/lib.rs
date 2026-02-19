pub mod a;
mod x; // x 是私有的，只有 lib 和它的后代能看
pub use x::X_PUB as X_PUB_RE; // 供 main.rs 调用
pub const LIB_PUB: &str = "LIB_PUB";
const LIB_PRIVATE: &str = "LIB_PRIVATE";

pub fn lib_visit() {
    println!(">>> LIB_VISIT (ROOT)");

    // 1. 访问子模块 a (公开)
    println!("a_pub: {}", a::A_PUB);
    // println!("{}", a::A_PRIVATE);           // ❌ 无法访问子模块私有成员

    // 2. 访问子模块 x (私有，但 lib 是父模块，有权进入)
    println!("x_pub: {}", x::X_PUB);
    // println!("{}", x::X_PRIVATE);           // ❌ 这个更不行

    // 3. 访问「孙」模块
    println!(": {}", a::b::B_PUB_CRATE);
    // println!(": {}", a::b::B_PRIVATE); // ❌ 儿子的隐私都看不到，更别想看孙子的
}