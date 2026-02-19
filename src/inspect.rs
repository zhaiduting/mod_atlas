/// 定义全局巡检宏
#[macro_export]
macro_rules! inspect {
    ($exp:expr) => {
        // 使用 stringify! 捕获表达式源码，由编译器求值
        println!("  {:>20} => {}", stringify!($exp).trim(), $exp);
    };
}