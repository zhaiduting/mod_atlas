/// 定义全局巡检宏
#[macro_export]
macro_rules! inspect {
    ($exp:expr) => {
        // 使用 stringify! 捕获表达式源码，由编译器求值
        println!("  {:>35} => {}", stringify!($exp).trim(), $exp);
    };

    // 专门处理已知无法访问的代码块
    ($exp:expr, X) => {
        println!("  {:>35} => error", stringify!($exp));
    };
}

#[macro_export]
macro_rules! header {
    () => {
        {
            let path = module_path!();
            println!("\n{:=^60}", "");
            println!(" 🌐 SCOPE: {}", path);
            println!("{:-^60}\n", "");
        }
    };
    ($msg:expr) => {
        {
            let path = module_path!();
            println!("\n{:=^60}", "");
            println!(" 🌐 SCOPE: {} ({})", path, $msg);
            println!("{:-^60}\n", "");
        }
    };
}