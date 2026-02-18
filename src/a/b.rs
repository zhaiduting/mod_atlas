pub fn b_pub() { println!("{}: b_pub", module_path!()); }
pub(super) fn b_pub_supper() {
    println!("{}: b_pub_supper", module_path!());
}
pub fn b_navigation_test() {
    println!("{}: b_navigation_test", module_path!());

    // 验证规则：super 向上跳 (b -> a)
    super::a_pub();
    super::super::call_from_lib();
    super::super::x::x_pub();

    // 验证规则：crate 绝对路径 (b -> lib -> a)
    crate::a::a_pub();
    // crate::x::x_private(); // 不能访问兄弟私有物

    // 验证规则：子看父私有 (b -> lib::root_private)
    // 注意：虽然 root_private 没标 pub，但子模块有权访问
    crate::root_utility_private();
}
