use mod_atlas::{call_from_lib, root_pub};
fn main() {
    call_from_lib();
    root_pub();
    // mod_atlas::root_private(); // ❌
    // mod_atlas::x::x_private();
    mod_atlas::x_pub();
}
