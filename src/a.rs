pub mod b; // 挂载 a/b.rs

pub fn a_pub() { println!("{}: a_pub", module_path!()); }
fn a_private() { println!("{}: a_private", module_path!()); }

// 验证规则：pub(crate) 仅项目内可见
pub(crate) fn a_pub_crate() {
    println!("{}: a_pub_crate (only for this crate)", module_path!());
}