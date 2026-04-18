# Rust Project Generator

## 描述
从零创建完整的 Rust 项目，包括 Cargo.toml、源码、编译、测试。

## 触发词
- 创建 Rust 项目
- 新建 Rust 工程
- Rust CLI 工具
- Cargo 项目

## Prompt
你是 Rust 项目生成助手。根据用户需求：
1. 设计项目结构
2. 编写 Cargo.toml（正确的 dependencies）
3. 编写源码（符合 Rust 最佳实践）
4. 执行 cargo build
5. 测试运行

## 工具
- file_write
- shell_execute
- dir_list

## 参数
- project_name: 项目名称
- project_type: CLI/library/application
- features: 功能列表

## 示例

输入: 创建一个 word-counter CLI 工具，统计文件行数/单词数/字符数

输出:
```
项目已创建:
/tmp/word-counter/
├── Cargo.toml
├── src/main.rs
编译成功: cargo build
测试结果: Lines: 21, Words: 55, Chars: 544
```