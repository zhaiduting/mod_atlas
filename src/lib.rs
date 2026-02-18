pub mod a; // 挂载 a.rs
mod x;     // 挂载 x.rs (不带 pub，main.rs 看不见 x)
pub use x::x_pub; // 重新导出，对外可见
// pub use x::x_private; // 无法导出模块内的私有成员
pub fn root_pub() { println!("{}: root_pub", module_path!()); }
fn root_private() { println!("{}: root_private", module_path!()); }

// 验证规则：向上看（子看父）
pub fn call_from_lib() {
    println!("{}: calling a::b...", module_path!());
    a::b::b_pub(); // 父看子，需要子模块 pub
    // a::b::b_pub_supper();
}

// 辅助验证子看父私有
fn root_utility_private() { println!("{}: root_utility_private", module_path!()); }
