use crate::position::Position;
#[doc = " Counts legal move paths of a given length."]
#[doc = ""]
#[doc = " Shorter paths (due to mate or stalemate) are not counted."]
#[doc = " Computing perft numbers is useful for comparing, testing and"]
#[doc = " debugging move generation correctness and performance."]
#[doc = ""]
#[doc = " The method used here is simply recursively enumerating the entire tree of"]
#[doc = " legal moves. While this is fine for testing there is much"]
#[doc = " faster specialized software."]
#[doc = ""]
#[doc = " Warning: Computing perft numbers can take a long time, even at moderate"]
#[doc = " depths. The simple recursive algorithm can also overflow the stack at"]
#[doc = " high depths, but this will only come into consideration in the rare case"]
#[doc = " that high depths are feasible at all."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " use shakmaty::{Chess, perft};"]
#[doc = ""]
#[doc = " let pos = Chess::default();"]
#[doc = " assert_eq!(perft(&pos, 1), 20);"]
#[doc = " assert_eq!(perft(&pos, 2), 400);"]
#[doc = " assert_eq!(perft(&pos, 3), 8902);"]
#[doc = " ```"]
pub fn perft<P: Position + Clone>(pos: &P, depth: u32) -> u64 {
    if depth < 1 {
        1
    } else {
        let moves = pos.legal_moves();
        if depth == 1 {
            moves.len() as u64
        } else {
            moves
                .iter()
                .map(|m| {
                    let mut child = pos.clone();
                    child.play_unchecked(m);
                    perft(&child, depth - 1)
                })
                .sum()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Chess;
    #[test]
    fn test_perft() {
        let pos = Chess::default();
        assert_eq!(perft(&pos, 0), 1);
        assert_eq!(perft(&pos, 1), 20);
    }
}
