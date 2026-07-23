<p align="center">
  <img width="150" src="https://avatars.githubusercontent.com/u/330779794?s=200&v=4">
</p>

<h1 align="center">NATP</h1>

<p align="center">A Rust-Native Automated Theorem Prover for Finite Sequences</p>

```rust
use natp::{claim, theorem, Seq};

theorem()
    .forall::<Seq<i32>>()
    .claim(claim!(|xs| reverse(reverse(xs)) == xs))
    .prove()?
    .require_proven()?;
```

The kernel produces checkable certificates. The bounded backend verifies claims
up to a length limit, and the SMT backend checks obligations with Z3.
