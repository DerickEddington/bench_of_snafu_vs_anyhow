### About

This is a program that runs benchmarks of common basic operations of using SNAFU
vs Anyhow (both of which are error-helping libraries for Rust), to get a sense
of how their speeds compare for these operations in isolation, where the purpose
of Anyhow overlaps with that of SNAFU (SNAFU has a broader purpose but aims to
support similar use-cases as Anyhow), and this is focused on the types
[`snafu::Whatever`](https://docs.rs/snafu/latest/snafu/struct.Whatever.html) vs
[`anyhow::Error`](https://docs.rs/anyhow/latest/anyhow/struct.Error.html)
(actually, this uses my tweak of `Whatever` to make backtrace capturing optional
like it is for `anyhow::Error`).

### How to run

```shell
cargo run --release -- --bench
```
Or:
```shell
FILTER='Vs|(Thin|Fat)-More|Empty'  # Or whatever regex you want.
cargo run --release -- --bench "$FILTER"
```
Or:
```shell
RUST_LIB_BACKTRACE=1  cargo run --release -- --bench ...
```
Etc.

### Note

In contrast to these benchmarks, in some other real-world situations, additional
computations would be done around when error-handling operations are, and so the
comparative speed of how these libraries do the same/similar operation is
sometimes only a very small fraction of the total cost of everything that's
being done, and so the impact of one library's approach being somewhat slower is
sometimes negligible for such situations.

### TODO

I'm not super familiar with these libraries nor with using them, nor with
Criterion.rs, and I might have missed how to setup the most-appropriate
comparisons of most-similar operations as well as they should be.  If so, I'd
like to improve this.  Feel free to submit PRs.
