//! # Calculator Pipeline
//!
//! ```text
//! &str → Vec<Token> → Vec<Token> → f32
//!  |        |             |        |
//! str     tokens       postfix   result
//!  ↑        ↑             ↑
//! lexer   parser      evaluator
//! ```
//!
//! Public API:
//!
//! ```rust
//! pub fn calculate(expr: &str) -> Result<f32, CalcError>
//! ```

pub mod calc;
