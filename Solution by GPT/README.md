# Solution by GPT

這裡放的是 `Exercise by GPT` 的完整參考解答。解法不是唯一答案，但每份解答都保留相同 tests，方便你用 `cargo test --bin <name>_sol` 驗證。

建議練習順序：先讀題目與 tests，自己完成 exercise，再回來對照 solution。對照時重點看 ownership、borrowing、String/Vec/HashMap 的資料流，而不只是看最後程式碼。

`23_conversions` 的解答共 30 題，對應 `as`、`From` / `Into`、`FromStr`、`TryFrom` / `TryInto`、`AsRef` / `AsMut` 五個主題；建議特別對照 fallback、typed error、integer narrowing、reference conversion 與 in-place mutation 的 trade-off。

`20_threads` 的解答共 24 題，對應 `spawn thread`、`join handles`、`channel`、`Mutex` 四個主題；建議特別對照 ownership 如何移入 thread、`JoinHandle::join()` 的資料流、sender clone/drop 對 receiver iteration 的影響，以及 `Arc<Mutex<T>>` 的 lock scope。
