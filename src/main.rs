use mod_atlas::{child, visit, sibling_visit_re};

fn main() {
    println!("--- MAIN_RS START ---");

    // 访问各模块的入口函数
    visit();
    child::visit();
    child::grandchild::visit();
    // mod_atlas::sibling::visit(); // ❌ 模块 sibling 是私有的，虽然它在 lib 里，但 main 看不见它
    sibling_visit_re();          // ✅ 但可以访问私有模块中重新导出的成员

    //  权限测试
    // println!("{}", mod_atlas::child::PRIVATE); // ❌
    // println!("{}", mod_atlas::sibling::PUBLIC); // ❌ 无法访问私有模块的任何成员

    println!();
}