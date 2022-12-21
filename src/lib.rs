#![doc = " A library for chess move generation."]
#![doc = ""]
#![doc = " # Examples"]
#![doc = ""]
#![doc = " Generate legal moves in the starting position:"]
#![doc = ""]
#![doc = " ```"]
#![doc = " use shakmaty::{Chess, Position};"]
#![doc = ""]
#![doc = " let pos = Chess::default();"]
#![doc = " let legals = pos.legal_moves();"]
#![doc = " assert_eq!(legals.len(), 20);"]
#![doc = " ```"]
#![doc = ""]
#![doc = " Play moves:"]
#![doc = ""]
#![doc = " ```"]
#![doc = " # use shakmaty::{Chess, Position};"]
#![doc = " use shakmaty::{Square, Move, Role};"]
#![doc = " #"]
#![doc = " # let pos = Chess::default();"]
#![doc = ""]
#![doc = " // 1. e4"]
#![doc = " let pos = pos.play(&Move::Normal {"]
#![doc = "     role: Role::Pawn,"]
#![doc = "     from: Square::E2,"]
#![doc = "     to: Square::E4,"]
#![doc = "     capture: None,"]
#![doc = "     promotion: None,"]
#![doc = " })?;"]
#![doc = " # Ok::<_, shakmaty::PlayError<_>>(())"]
#![doc = " ```"]
#![doc = ""]
#![doc = " Detect game end conditions:"]
#![doc = ""]
#![doc = " ```"]
#![doc = " # use shakmaty::{Chess, Position};"]
#![doc = " # let pos = Chess::default();"]
#![doc = " assert!(!pos.is_checkmate());"]
#![doc = " assert!(!pos.is_stalemate());"]
#![doc = " assert!(!pos.is_insufficient_material());"]
#![doc = " assert_eq!(pos.outcome(), None); // no winner yet"]
#![doc = " ```"]
#![doc = ""]
#![doc = " Also supports [FEN](fen), [SAN](san) and"]
#![doc = " [UCI](uci) formats for positions and moves."]
#![doc = ""]
#![doc = " # Feature flags"]
#![doc = ""]
#![doc = " * `alloc`: Enables APIs which require the"]
#![doc = "   [`alloc`](https://doc.rust-lang.org/stable/alloc/index.html) crate"]
#![doc = "   (e.g. FEN string rendering)."]
#![doc = " * `std`: Implements the"]
#![doc = "   [`std::error::Error`](https://doc.rust-lang.org/stable/std/error/trait.Error.html)"]
#![doc = "   trait for various errors in the crate."]
#![doc = "   Implies the `alloc` feature (since `std` depends on `alloc` anyway)."]
#![doc = "   Enabled by default for convenience. For `no_std` environments, this must"]
#![doc = "   be disabled with `default-features = false`."]
#![doc = " * `variant`: Enables support for all Lichess variants."]
#![doc = " * `step`: Implements"]
#![doc = "   [`std::iter::Step`](https://doc.rust-lang.org/nightly/std/iter/trait.Step.html)"]
#![doc = "   for [`Square`], [`File`], and [`Rank`]. Requires nightly Rust."]
#![doc = " * `nohash-hasher`: Implements"]
#![doc = "   [`nohash_hasher::IsEnabled`](https://docs.rs/nohash-hasher/0.2/nohash_hasher/trait.IsEnabled.html)"]
#![doc = "   for sensible types."]
#![no_std]
#![doc(html_root_url = "https://docs.rs/shakmaty/0.23.0")]
#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(missing_debug_implementations)]
#![cfg_attr(feature = "step", feature(step_trait))]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
pub mod attacks;
pub mod bitboard;
pub mod board;
mod color;
pub mod fen;
mod magics;
mod movelist;
mod perft;
mod position;
mod role;
pub mod san;
mod setup;
mod square;
mod types;
pub mod uci;
mod util;
#[cfg(feature = "variant")]
pub mod variant;
pub mod zobrist;
pub use bitboard::Bitboard;
pub use board::Board;
pub use color::{ByColor, Color, ParseColorError};
pub use movelist::MoveList;
pub use perft::perft;
pub use position::{
    Chess, FromSetup, Outcome, ParseOutcomeError, PlayError, Position, PositionError,
    PositionErrorKinds,
};
pub use role::{ByRole, Role};
pub use setup::{Castles, Setup};
pub use square::{File, ParseSquareError, Rank, Square};
pub use types::{CastlingMode, CastlingSide, EnPassantMode, Move, Piece, RemainingChecks};
#[cfg(feature = "nohash-hasher")]
impl nohash_hasher::IsEnabled for File {}
#[cfg(feature = "nohash-hasher")]
impl nohash_hasher::IsEnabled for Rank {}
#[cfg(feature = "nohash-hasher")]
impl nohash_hasher::IsEnabled for Square {}
#[cfg(feature = "nohash-hasher")]
impl nohash_hasher::IsEnabled for Role {}
#[cfg(feature = "nohash-hasher")]
impl nohash_hasher::IsEnabled for Color {}
