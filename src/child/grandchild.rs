// 本模块的变量定义
const PRIVATE: &str = "ok";
pub const PUBLIC: &str = "ok";
pub(crate) const PUB_CRATE: &str = "ok";
pub(super) const PUB_SUPER: &str = "ok";

pub fn visit() {
    header!("Deep Mod");

    // 1. 访问父模块肯定没问题（在 child 中已验证）
    // 2. 访问「祖父」 
    inspect!(super::super::PUBLIC); // 啰嗦
    inspect!(crate::PRIVATE); // ✅ 爷对孙亦无隐私

    // 3. 访问「远亲」 
    inspect!(crate::sibling::PUBLIC);
    // inspect!(crate::sibling::PRIVATE); // ❌ 无法访问「父之兄弟」模块的私有成员
    inspect!(crate::sibling::PRIVATE, X); // ❌ 无法访问「父之兄弟」模块的私有成员
    // inspect!(super::super::sibling::PRIVATE); // ❌ 换种写法照样不行
    inspect!(super::super::sibling::PRIVATE, X); // ❌ 换种写法照样不行
}