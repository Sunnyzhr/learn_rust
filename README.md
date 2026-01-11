# Rust Fast Track 学习指南

这是一个 Rust 基础教学项目的代码仓库，包含 8 个主题 (Topic)，涵盖了 Rust 的核心概念。

## 目录结构

项目是一个 Cargo Workspace，包含以下成员：

- `topic01_basics`: 基础语法（结构体、枚举、模式匹配）
- `topic02_slices`: 切片（字符串切片、数组切片）
- `topic03_ownership`: 所有权与借用（移动语义、引用）
- `topic04_lifetimes`: 生命周期
- `topic05_traits_dyn_drop`: Trait、Trait Objects (dyn) 和 Drop
- `topic06_closures_errors`: 闭包与错误处理
- `topic07_smart_pointers`: 智能指针 (Box, Rc, RefCell)
- `topic08_modules`: 模块系统

## 如何运行代码

你可以通过 `cargo run -p <package_name>` 命令运行特定的主题。

### 1. 编译所有项目

首先，确保所有代码都能通过编译：

```bash
cargo build
```

### 2. 运行各个主题

请按顺序运行以下命令查看各个主题的输出结果：

**Topic 01: 基础**
```bash
cargo run -p topic01_basics
```

**Topic 02: 切片**
```bash
cargo run -p topic02_slices
```

**Topic 03: 所有权**
```bash
cargo run -p topic03_ownership
```

**Topic 04: 生命周期**
```bash
cargo run -p topic04_lifetimes
```

**Topic 05: Traits & Drop**
```bash
cargo run -p topic05_traits_dyn_drop
```

**Topic 06: 闭包与错误处理**
```bash
cargo run -p topic06_closures_errors
```

**Topic 07: 智能指针**
```bash
cargo run -p topic07_smart_pointers
```

**Topic 08: 模块系统**
```bash
cargo run -p topic08_modules
```

## 常见问题

在编译过程中，你可能会看到一些 `warning: unused variable`（未使用变量）的警告。这是正常的，因为教学代码中经常包含一些定义了但未在主流程中使用的示例变量或函数。这些警告不会影响程序的运行。
