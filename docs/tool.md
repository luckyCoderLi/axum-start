
## cargo

1. cargo-watch
自动监听文件变更进行编译

```
cargo install cargo-watch

cargo watch -q -c -w src/ -x run

cargo watch -q -c -w tests/ -x "tes
t -q quick_dev -- --nocapture"
```