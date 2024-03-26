# naive-raft

The [Raft](https://raft.github.io/) consensus algorithm used for building fault-tolerant distributed systems.
You should try to understand the concept from whatever resources possible and complete the following task.

## Task details

The task at hand is a simplified and abstracted version of Raft.

In this task, you are expected to implement the `RaftNode` trait for `Node` to handle the situations of timeout and receiving a message.
The code is well documented but if you have any questions regarding the code provided, you should reach out ASAP.

## Expected Outcome

- We have provided a simple test for you but you should add tests as you see fit, the tests should cover different scenarios, ranging 2 or more nodes.
- You can implement the same framework in Golang or Rust
- You should rewrite the existing `connect_nodes` function with the `easy_connect!` macro in `src/helpers` and use it in the tests.

_Note:_ Please do not share the content of this repository with anyone else.

## References

[1]. Raft implementation in Rust: [rust-rs](https://github.com/tikv/raft-rs)

## Test Results

The tests can be executed by the following command:-

```sh
cargo test --package rs-raft-interview --lib -- tests --nocapture
```

The output will look something like:-

```sh
running 3 tests
test tests::followers_should_replicate_leader_logs ... ok
test tests::should_not_re_elect_if_already_elected_for_term ... ok
test tests::should_elect_new_leader_on_timeout ... ok
```
