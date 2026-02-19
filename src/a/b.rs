// 本模块的变量定义
const B_PRIVATE: &str = "B_PRIVATE";
pub const B_PUB: &str = "B_PUB";
pub(crate) const B_PUB_CRATE: &str = "B_PUB_CRATE";
pub(super) const B_PUB_SUPER: &str = "B_PUB_SUPER";

pub fn b_visit() {
    println!("\n>>> B_VISIT (Deep)");

    // 1. 访问父模块肯定没问题（在 a.rs 中已验证）
    // 2. 访问「祖父」 
    inspect!( super::super::LIB_PUB); // 啰嗦
    inspect!(  crate::LIB_PRIVATE); // ✅ 爷对孙亦无隐私

    // 3. 访问「远亲」 
    inspect!( crate::x::X_PUB);
    // inspect!( crate::x::X_PRIVATE); // ❌ 无法访问「父之兄弟」模块的私有成员
    // inspect!( super::super::x::X_PRIVATE); // ❌ 换种写法照样不行
}