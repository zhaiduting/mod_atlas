pub fn x_pub() {
    println!("{}: x_pub", module_path!());

    // 验证规则：兄弟互访 (x -> a)
    crate::a::a_pub();
    // crate::a::a_private(); // ❌
}

fn x_private() {
    println!("{}: x_private", module_path!());
}
