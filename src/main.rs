use mod_atlas::{a, lib_visit, LIB_PUB};

fn main() {
    println!("--- MAIN_RS START ---");

    // 访问各模块的入口函数
    lib_visit();
    a::a_visit();
    a::b::b_visit();
    // mod_atlas::x::x_visit(); // ❌ 模块 x 是私有的，虽然它在 lib 里，但 main 看不见它

    // 访问变量
    // println!("Direct access: {}, {}", LIB_PUB, X_PUB); // ✅

    // ❌ 权限测试
    // println!("{}", mod_atlas::a::A_PRIVATE); // ❌
    // println!("{}", mod_atlas::x::X_PRIVATE); // ❌
}