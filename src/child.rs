pub mod grandchild;

pub const PUBLIC: &str = "ok";
const PRIVATE: &str = "ok";

pub fn visit() {
    header!("Sub Mod");

    // 1. 访问父模块 lib
    inspect!(super::PUBLIC);
    inspect!(super::PRIVATE);     // ✅父辈隐私透明化，可访

    // 2. 访问子模块 b
    inspect!(grandchild::PUB_CRATE);
    inspect!(grandchild::PUB_SUPER);
    // inspect!(grandchild::PRIVATE);            // ❌ 无法访问子模块的私有成员
    inspect!(grandchild::PRIVATE, X);            // ❌ 无法访问子模块的私有成员

    // 3. 访问兄弟模块
    // 虽然 sibling 在 lib 里是私有的，但 child 可以通过 super::sibling 访问它（符合「父对子无隐私」的规则）
    // child 可以进一步访问 super::sibling::PUBLIC
    // child 不可以进一步访问 super::sibling::PRIVATE（符合「兄弟之间有隐私」的规则）
    inspect!(super::sibling::PUBLIC);         // 兄弟 pub 可访
    // inspect!(crate::sibling::PRIVATE);     // ❌ 兄弟隐私不可访
    inspect!(crate::sibling::PRIVATE, X);     // ❌ 兄弟隐私不可访
}