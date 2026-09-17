# quadratic-rust

A small command-line program that solves quadratic equations of the form `ax² + bx + c = 0`.

Written in Rust as a learning project.

## Usage

```bash
cargo run
```

The program asks for the three integer coefficients and prints the roots:

```
Enter a:
1
Enter b:
-5
Enter c:
6
Two roots: x1=3, x2=2.
```

Invalid input is rejected and the prompt is repeated. The coefficient `a` must not be zero — otherwise the equation is not quadratic and the program panics.

## How it works

The discriminant `d = b² - 4ac` decides the outcome:

| Discriminant | Result                                     |
| ------------ | ------------------------------------------ |
| `d > 0`      | two distinct real roots                    |
| `d = 0`      | one real root                              |
| `d < 0`      | no real roots                              |

## Project layout

| File              | Contents                                                          |
| ----------------- | ----------------------------------------------------------------- |
| `src/main.rs`     | CLI: reads the coefficients and prints the result                 |
| `src/equation.rs` | `QuadraticEquation` type and the `Solution` enum                  |
| `src/solver.rs`   | `solve()` and the discriminant calculation                        |

## Development

Formatting follows `rustfmt.toml` (`max_width = 100`). Before committing, run:

```bash
cargo fmt
cargo clippy --all-targets -- -W clippy::pedantic -D warnings
```

Requires a Rust toolchain with the `rustfmt` and `clippy` components (`rustup component add rustfmt clippy`).
