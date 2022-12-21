use crate::{
    color::{ByColor, Color},
    role::Role,
    square::{File, Square},
    util::overflow_error,
};
use core::{
    fmt::{self, Write as _},
    num,
};
#[doc = " A piece with [`Color`] and [`Role`]."]
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}
impl Piece {
    pub fn char(self) -> char {
        self.color.fold_wb(self.role.upper_char(), self.role.char())
    }
    pub fn from_char(ch: char) -> Option<Piece> {
        Role::from_char(ch).map(|role| role.of(Color::from_white(32 & ch as u8 == 0)))
    }
}
#[doc = " Information about a move."]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[repr(align(4))]
pub enum Move {
    Normal {
        role: Role,
        from: Square,
        capture: Option<Role>,
        to: Square,
        promotion: Option<Role>,
    },
    EnPassant {
        from: Square,
        to: Square,
    },
    Castle {
        king: Square,
        rook: Square,
    },
    Put {
        role: Role,
        to: Square,
    },
}
impl Move {
    #[doc = " Gets the role of the moved piece."]
    pub const fn role(&self) -> Role {
        match *self {
            Move::Normal { role, .. } | Move::Put { role, .. } => role,
            Move::EnPassant { .. } => Role::Pawn,
            Move::Castle { .. } => Role::King,
        }
    }
    #[doc = " Gets the origin square or `None` for drops."]
    pub const fn from(&self) -> Option<Square> {
        match *self {
            Move::Normal { from, .. } | Move::EnPassant { from, .. } => Some(from),
            Move::Castle { king, .. } => Some(king),
            Move::Put { .. } => None,
        }
    }
    #[doc = " Gets the target square. For castling moves this is the corresponding"]
    #[doc = " rook square."]
    pub const fn to(&self) -> Square {
        match *self {
            Move::Normal { to, .. } | Move::EnPassant { to, .. } | Move::Put { to, .. } => to,
            Move::Castle { rook, .. } => rook,
        }
    }
    #[doc = " Gets the role of the captured piece or `None`."]
    pub const fn capture(&self) -> Option<Role> {
        match *self {
            Move::Normal { capture, .. } => capture,
            Move::EnPassant { .. } => Some(Role::Pawn),
            _ => None,
        }
    }
    #[doc = " Checks if the move is a capture."]
    pub const fn is_capture(&self) -> bool {
        matches!(
            *self,
            Move::Normal {
                capture: Some(_),
                ..
            } | Move::EnPassant { .. }
        )
    }
    #[doc = " Checks if the move is en passant."]
    pub const fn is_en_passant(&self) -> bool {
        matches!(*self, Move::EnPassant { .. })
    }
    #[doc = " Checks if the move zeros the half-move clock."]
    pub const fn is_zeroing(&self) -> bool {
        matches!(
            *self,
            Move::Normal {
                role: Role::Pawn,
                ..
            } | Move::Normal {
                capture: Some(_),
                ..
            } | Move::EnPassant { .. }
                | Move::Put {
                    role: Role::Pawn,
                    ..
                }
        )
    }
    #[doc = " Gets the castling side."]
    pub fn castling_side(&self) -> Option<CastlingSide> {
        match *self {
            Move::Castle { king, rook } if king < rook => Some(CastlingSide::KingSide),
            Move::Castle { .. } => Some(CastlingSide::QueenSide),
            _ => None,
        }
    }
    #[doc = " Checks if the move is a castling move."]
    pub const fn is_castle(&self) -> bool {
        matches!(*self, Move::Castle { .. })
    }
    #[doc = " Gets the promotion role."]
    pub const fn promotion(&self) -> Option<Role> {
        match *self {
            Move::Normal { promotion, .. } => promotion,
            _ => None,
        }
    }
    #[doc = " Checks if the move is a promotion."]
    pub const fn is_promotion(&self) -> bool {
        matches!(
            *self,
            Move::Normal {
                promotion: Some(_),
                ..
            }
        )
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Move::Normal {
                role,
                from,
                capture,
                to,
                promotion,
            } => {
                if role != Role::Pawn {
                    f.write_char(role.upper_char())?;
                }
                write!(
                    f,
                    "{}{}{}",
                    from,
                    if capture.is_some() { 'x' } else { '-' },
                    to
                )?;
                if let Some(p) = promotion {
                    write!(f, "={}", p.upper_char())?;
                }
                Ok(())
            }
            Move::EnPassant { from, to, .. } => write!(f, "{from}x{to}"),
            Move::Castle { king, rook } => f.write_str(if king < rook { "O-O" } else { "O-O-O" }),
            Move::Put { role, to } => {
                if role != Role::Pawn {
                    f.write_char(role.upper_char())?;
                }
                write!(f, "@{to}")
            }
        }
    }
}
#[doc = " `KingSide` (O-O) or `QueenSide` (O-O-O)."]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub enum CastlingSide {
    KingSide = 0,
    QueenSide = 1,
}
impl CastlingSide {
    pub const fn is_queen_side(self) -> bool {
        match self {
            CastlingSide::KingSide => false,
            CastlingSide::QueenSide => true,
        }
    }
    pub const fn is_king_side(self) -> bool {
        !self.is_queen_side()
    }
    pub const fn from_queen_side(queen_side: bool) -> CastlingSide {
        if queen_side {
            CastlingSide::QueenSide
        } else {
            CastlingSide::KingSide
        }
    }
    pub const fn from_king_side(king_side: bool) -> CastlingSide {
        if king_side {
            CastlingSide::KingSide
        } else {
            CastlingSide::QueenSide
        }
    }
    pub const fn king_to_file(self) -> File {
        match self {
            CastlingSide::KingSide => File::G,
            CastlingSide::QueenSide => File::C,
        }
    }
    pub const fn rook_to_file(self) -> File {
        match self {
            CastlingSide::KingSide => File::F,
            CastlingSide::QueenSide => File::D,
        }
    }
    pub fn king_to(self, color: Color) -> Square {
        Square::from_coords(self.king_to_file(), color.backrank())
    }
    pub fn rook_to(self, color: Color) -> Square {
        Square::from_coords(self.rook_to_file(), color.backrank())
    }
    #[doc = " `KingSide` and `QueenSide`, in this order."]
    pub const ALL: [CastlingSide; 2] = [CastlingSide::KingSide, CastlingSide::QueenSide];
}
#[doc = " `Standard` or `Chess960`."]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub enum CastlingMode {
    Standard,
    Chess960,
}
impl CastlingMode {
    pub const fn from_standard(standard: bool) -> CastlingMode {
        if standard {
            CastlingMode::Standard
        } else {
            CastlingMode::Chess960
        }
    }
    pub const fn from_chess960(chess960: bool) -> CastlingMode {
        if chess960 {
            CastlingMode::Chess960
        } else {
            CastlingMode::Standard
        }
    }
    pub const fn is_standard(self) -> bool {
        matches!(self, CastlingMode::Standard)
    }
    pub const fn is_chess960(self) -> bool {
        matches!(self, CastlingMode::Chess960)
    }
}
#[doc = " When to include the en passant square."]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub enum EnPassantMode {
    #[doc = " Only if there is a fully legal en passant move."]
    Legal,
    #[doc = " Only when a pawn has been advanced by two steps and there is an"]
    #[doc = " enemy pawn next to it."]
    #[doc = ""]
    #[doc = " Follows the X-FEN specification."]
    PseudoLegal,
    #[doc = " Whenever a pawn has been advanced by two steps."]
    #[doc = ""]
    #[doc = " Follows the FEN specification."]
    Always,
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;
    #[test]
    fn test_role_order() {
        assert!(Role::Pawn < Role::Knight);
        assert!(Role::Knight < Role::Bishop);
        assert!(Role::Bishop < Role::Rook);
        assert!(Role::Rook < Role::Queen);
        assert!(Role::Queen < Role::King);
    }
    #[test]
    fn test_size() {
        assert!(mem::size_of::<Move>() <= 8);
    }
}
#[doc = " The number of checks the respective side needs to give in order to win"]
#[doc = " (in a game of Three-Check)."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use shakmaty::{ByColor, RemainingChecks};"]
#[doc = ""]
#[doc = " let mut remaining_checks = ByColor::<RemainingChecks>::default();"]
#[doc = " assert_eq!(remaining_checks.white, RemainingChecks::new(3));"]
#[doc = " assert_eq!(remaining_checks.black, RemainingChecks::new(3));"]
#[doc = ""]
#[doc = " for _ in 0..5 {"]
#[doc = "     remaining_checks.white = remaining_checks.white.saturating_sub(1);"]
#[doc = " }"]
#[doc = ""]
#[doc = " assert!(remaining_checks.white.is_zero());"]
#[doc = " ```"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
#[repr(C)]
pub struct RemainingChecks(u32);
impl Default for RemainingChecks {
    fn default() -> RemainingChecks {
        RemainingChecks(3)
    }
}
impl RemainingChecks {
    #[doc = " Constructs a new [`RemainingChecks`] value."]
    #[doc = ""]
    #[doc = " # Panics"]
    #[doc = ""]
    #[doc = " Panics if `n > 3`."]
    pub const fn new(n: u32) -> RemainingChecks {
        assert!(n <= 3);
        RemainingChecks(n)
    }
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
    #[must_use]
    pub const fn saturating_sub(self, n: u32) -> RemainingChecks {
        RemainingChecks(self.0.saturating_sub(n))
    }
}
macro_rules ! int_from_remaining_checks_impl { ($ ($ t : ty) +) => { $ (impl From < RemainingChecks > for $ t { # [inline] fn from (RemainingChecks (checks) : RemainingChecks) -> $ t { checks as $ t } }) + } }
int_from_remaining_checks_impl! { u8 i8 u16 i16 u32 i32 u64 i64 usize isize }
macro_rules ! try_remaining_checks_from_int_impl { ($ ($ t : ty) +) => { $ (impl core :: convert :: TryFrom <$ t > for RemainingChecks { type Error = num :: TryFromIntError ; # [inline] fn try_from (value : $ t) -> Result < RemainingChecks , Self :: Error > { let n = u32 :: try_from (value) ?; if n <= 3 { Ok (RemainingChecks :: new (n)) } else { Err (overflow_error ()) } } }) + } }
try_remaining_checks_from_int_impl! { u8 i8 u16 i16 u32 i32 u64 i64 usize isize }
impl fmt::Display for ByColor<RemainingChecks> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}", self.white.0, self.black.0)
    }
}
#[no_mangle]
pub extern "C" fn ffi_piece_char(self_: Piece) -> char {
    <Piece>::char(self_)
}
#[no_mangle]
pub extern "C" fn ffi_piece_from_char(ch: char) -> Option<Piece> {
    <Piece>::from_char(ch)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_is_queen_side(self_: CastlingSide) -> bool {
    <CastlingSide>::is_queen_side(self_)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_is_king_side(self_: CastlingSide) -> bool {
    <CastlingSide>::is_king_side(self_)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_from_queen_side(queen_side: bool) -> CastlingSide {
    <CastlingSide>::from_queen_side(queen_side)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_from_king_side(king_side: bool) -> CastlingSide {
    <CastlingSide>::from_king_side(king_side)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_king_to_file(self_: CastlingSide) -> File {
    <CastlingSide>::king_to_file(self_)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingside_rook_to_file(self_: CastlingSide) -> File {
    <CastlingSide>::rook_to_file(self_)
}
#[no_mangle]
pub extern "C" fn ffi_castlingside_king_to(self_: CastlingSide, color: Color) -> Square {
    <CastlingSide>::king_to(self_, color)
}
#[no_mangle]
pub extern "C" fn ffi_castlingside_rook_to(self_: CastlingSide, color: Color) -> Square {
    <CastlingSide>::rook_to(self_, color)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingmode_from_standard(standard: bool) -> CastlingMode {
    <CastlingMode>::from_standard(standard)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingmode_from_chess960(chess960: bool) -> CastlingMode {
    <CastlingMode>::from_chess960(chess960)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingmode_is_standard(self_: CastlingMode) -> bool {
    <CastlingMode>::is_standard(self_)
}
#[no_mangle]
pub const extern "C" fn ffi_castlingmode_is_chess960(self_: CastlingMode) -> bool {
    <CastlingMode>::is_chess960(self_)
}
#[no_mangle]
pub const extern "C" fn ffi_remainingchecks_is_zero(self_: RemainingChecks) -> bool {
    <RemainingChecks>::is_zero(self_)
}
