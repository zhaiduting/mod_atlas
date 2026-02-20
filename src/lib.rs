// 加载 macros.rs 并把里面定义的所有宏‘全局化’
#[macro_use]
mod macros;

pub mod child;
mod sibling; // x 是私有的，只有 lib 和它的后代能看
pub use sibling::visit as sibling_visit_re; // 供 main.rs 调用
pub const PUBLIC: &str = "ok";
const PRIVATE: &str = "ok";

pub fn visit() {
    header!("Crate Root");
    // 1. 访问子模块 child (公开)
    inspect!(child::PUBLIC);
    // inspect!(child::PRIVATE);           // ❌ 无法访问子模块私有成员
    inspect!(child::PRIVATE, X);           // ❌ 无法访问子模块私有成员

    // 2. 访问子模块 sibling (私有，但 lib 是父模块，有权进入)
    inspect!(sibling::PUBLIC);
    // inspect!(sibling::PRIVATE);           // ❌ 这个更不行
    inspect!(sibling::PRIVATE, X);           // ❌ 这个更不行

    // 3. 访问「孙」模块
    inspect!(child::grandchild::PUB_CRATE);
    // inspect!(child::grandchild::PRIVATE); // ❌ 儿子的隐私都看不到，更别想看孙子的
    inspect!(child::grandchild::PRIVATE, X); // ❌ 儿子的隐私都看不到，更别想看孙子的
}