workflow "test" {
  on = "push"
  resolves = ["cargo test"]
}

action "cargo test" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check && cargo test"
}
