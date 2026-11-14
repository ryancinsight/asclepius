# Milestone ladder ([ADR 0002](../docs/adr/0002-release-scope-1-0.md))
<a id="M-law-oracles"></a> **M-law-oracles** (0.4.0) needs: (none) — the laws stand on independent oracles and cited sources.
- exit: pass law oracle tests (`cargo test -p asclepius --locked --all-features` passes, including the TCP/NTCP, Arrhenius, gEUD, and gradient cross-check anchors).
- standing: no open correctness item.
<a id="M-gates-docs"></a> **M-gates-docs** (0.5.0) needs: M-law-oracles — CI, book, and README gates execute what they claim.
- exit: pass workspace gates locked (`cargo test --workspace --locked --all-features` passes; every cargo gate uses the committed lockfile; semver, MSRV, and Rustdoc gates run).
- exit: book tests execute (`mdbook test docs/book` compiles every Rust fence).
- standing: no open correctness item.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-gates-docs — stable law release (authority-gated).
- standing: `cargo fmt --all -- --check`, `cargo clippy --workspace --locked --all-targets --all-features -- -D warnings`, `cargo test --workspace --locked --all-features`, `cargo test --workspace --locked --doc --all-features`, and `cargo doc --workspace --locked --no-deps --all-features` pass; SemVer and publish dry-run gates required for declared releases pass; README and rustdoc are current; no open correctness item.
