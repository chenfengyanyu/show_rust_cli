# show_rust_cli
Command Line Applications in Rust

## log 打印 | [文档地址](https://docs.rs/env_logger/0.9.0/env_logger/index.html)
使用说明
```
fn user_log() {
    env_logger::init();
    trace!("Progress Bar is start!");
    thread::sleep(Duration::from_secs(1));
    info!("Time is out!");
    thread::sleep(Duration::from_secs(1));
    warn!("oops, nothing implemented!");
}
```
运行命令，需要带上环境变量，否则无任何输出
```bash
# Even setting max level to ‘trace’ or DEBUG_LOG=trace
RUST_LOG=trace cargo run -- main src/main.rs
```
效果演示
![show](https://github.com/chenfengyanyu/show_rust_cli/blob/main/src/image/1.png)


