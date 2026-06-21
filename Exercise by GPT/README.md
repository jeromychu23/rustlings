# Exercise by GPT

這是一組根據 rustlings `00_intro` 到 `18_iterators`、`quiz1`、`quiz2` 進度延伸的 Rust 練習題。

## 使用方式

- 多數主題 6 題：初級 2 題、中級 2 題、進階 2 題。
- `15_traits` 是重點主題，共 10 題；進階題會需要寫較完整的 struct、impl block、trait implementation 和 pipeline logic。
- `18_iterators` 共 10 題：Beginner 3 題、Intermediate 4 題、Advanced 3 題，涵蓋 iterator state、adapters、`Option` / `Result`、single-pass aggregation 和自訂 `Iterator`。
- Iterator 題目的指定函式不得使用 `for`、`while` 或手動 index traversal；第 9、10 題也不得先建立 intermediate collection。
- 題目偏向軟體工程場景：Backend/API、CLI tooling、networking、infrastructure、security/correctness。
- 先嘗試完成 `Exercise by GPT` 裡的函式，再對照 `Solution by GPT`。
- 題目可用 `cargo check --bin gpt_<topic>_<nn>` 檢查是否可編譯。
- 解答可用 `cargo test --bin gpt_<topic>_<nn>_sol` 驗證。
- Iterator 範例：`cargo check --bin gpt_iterators_01`、`cargo test --bin gpt_iterators_01_sol`。
- `gpt_iterators_06`、`07`、`10` 的初始 compiler error 是題目設計的一部分。

## 主題

00 Intro, 01 Variables, 02 Functions, 03 If, 04 Primitive Types, 05 Vecs, 06 Move Semantics, 07 Structs, 08 Enums, 09 Strings, 10 Modules, 11 HashMaps, 14 Generics, 15 Traits, 18 Iterators, Quiz 1 Review, Quiz 2 Review.
