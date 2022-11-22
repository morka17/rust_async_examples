## Poll
```rust
pub enum Poll<T> {
  Ready(T),
  Pending,
}
```




## Async Executors
Futures in action we need a way to run them. What calls the poll method? That is the job of an executor. 