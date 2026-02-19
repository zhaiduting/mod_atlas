use mod_atlas::{a, lib_visit, LIB_PUB, x_visit_re};

fn main() {
    println!("--- MAIN_RS START ---");

    // 访问各模块的入口函数
    lib_visit();
    a::a_visit();
    a::b::b_visit();
    // mod_atlas::x::x_visit(); // ❌ 模块 x 是私有的，虽然它在 lib 里，但 main 看不见它

    //  权限测试
    // println!("{}", mod_atlas::a::A_PRIVATE); // ❌
    // println!("{}", mod_atlas::x::X_PUB); // ❌ 无法访问私有模块的任何成员
    x_visit_re() // ✅ 但可以访问私有模块中重新导出的成员
}