pub const PUBLIC: &str = "ok";
const PRIVATE: &str = "ok";

pub fn visit() {
    header!("Private Sub-mod");

    // 1. 访问父模块 lib
    inspect!(super::PUBLIC);
    inspect!(super::PRIVATE);

    // 2. 访问兄弟模块 child
    inspect!(super::child::PUBLIC);
    inspect!(super::child::grandchild::PUBLIC);
}