# calculation
基于 Rust 实现的**数学表达式解析计算器**，支持交互式 REPL 与命令行直接计算，包含完整的词法分析、语法解析与表达式求值功能。

---

## 项目介绍
这是一个用 Rust 编写的表达式计算器玩具项目，实现了从字符串输入到语法树构建、再到结果计算的完整流程，适合学习 Rust、编译器前端与表达式解析原理。

功能包括：
- 交互式命令行（REPL）
- 基础算术运算（加、减、乘、除、取模）
- 负数、括号、优先级与结合性
- 数学常量（pi、e）
- 三角函数（sin、cos、tan）
- 平方根、幂运算
- 完善的错误处理

---

## 项目结构
```
calculation/
├── src/
│   ├── bin/
│   │   └── main.rs          # 程序入口：REPL 交互/命令行执行器
│   ├── error.rs             # 错误类型定义与统一处理
│   ├── expression.rs        # 表达式枚举与求值逻辑
│   ├── lexer.rs             # 词法分析器（Token 生成器）
│   ├── lib.rs               # 库根模块：统一导出公共接口
│   ├── operator.rs          # 运算符定义（优先级、结合性）
│   ├── parser.rs            # 递归下降语法解析器
│   └── token.rs             # Token 枚举定义
├── tests/                   # 单元测试与集成测试用例
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

---

## 快速开始
### 环境要求
- Rust 工具链（cargo）

### 运行
```bash
cargo run
```

## 使用示例
```
> 1+2*3
7

> pi/2
1.5707963267948966

> sin(pi/2)
1

> sqrt(16)
4

> (10-2)*3
24
```

退出：
```
> exit
```

---

## 支持的运算符
- 加法 `+`、减法 `-`
- 乘法 `*`、除法 `/`、取模 `%`
- 幂运算 `^`
- 负号 `-`
- 括号 `()`

---

## 支持的函数与常量
- 常量：`pi`、`e`
- 三角函数：`sin`、`cos`、`tan`
- 平方根：`sqrt`

---

## 构建
```bash
cargo build --release
./target/release/calculation
```

---

## 测试
```bash
cargo test
```

---

## 说明
本项目为学习型项目，用于理解：
- 词法分析（Lexer）
- 语法解析（Parser）
- 表达式求值（Evaluator）
- Rust 生命周期、错误处理、模块化设计

---