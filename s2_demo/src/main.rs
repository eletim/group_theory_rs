// src/main.rs

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum S2 {
    /// e
    E,
    /// (1 2)
    T,
}

impl S2 {
    /// Apply this permutation to an element in {1, 2}.
    fn apply(self, x: u8) -> u8 {
        match (self, x) {
            (S2::E, 1) => 1,
            (S2::E, 2) => 2,
            (S2::T, 1) => 2,
            (S2::T, 2) => 1,
            _ => panic!("S2 only acts on {{1,2}}, but got x={}", x),
        }
    }

    /// Composition: self ∘ other (apply other first, then self).
    fn compose(self, other: S2) -> S2 {
        // For S2 there are only 4 cases; match table is simplest and bug-proof.
        match (self, other) {
            (S2::E, S2::E) => S2::E,
            (S2::E, S2::T) => S2::T,
            (S2::T, S2::E) => S2::T,
            (S2::T, S2::T) => S2::E,
        }
    }

    fn id() -> S2 {
        S2::E
    }

    fn inv(self) -> S2 {
        // In S2: e^-1 = e, (12)^-1 = (12)
        self
    }
}

fn main() {
    let e = S2::E;
    let t = S2::T;

    // Demo: (12) swaps 1 and 2
    println!("t(1) = {}", t.apply(1)); // 2
    println!("t(2) = {}", t.apply(2)); // 1

    // Demo: t ∘ t = e
    let tt = t.compose(t);
    println!("t ∘ t = {:?}", tt); // E

    // Demo: (t ∘ e)(1) = t(1)
    let te = t.compose(e);
    println!("(t ∘ e)(1) = {}", te.apply(1)); // 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(S2::E.apply(1), 1);
        assert_eq!(S2::E.apply(2), 2);
        assert_eq!(S2::T.apply(1), 2);
        assert_eq!(S2::T.apply(2), 1);
    }

    #[test]
    fn test_composition_table() {
        let e = S2::E;
        let t = S2::T;

        assert_eq!(e.compose(e), e);
        assert_eq!(e.compose(t), t);
        assert_eq!(t.compose(e), t);
        assert_eq!(t.compose(t), e);
    }

    #[test]
    fn test_identity_law() {
        let e = S2::id();
        for g in [S2::E, S2::T] {
            assert_eq!(g.compose(e), g); // g ∘ e = g
            assert_eq!(e.compose(g), g); // e ∘ g = g
        }
    }

    #[test]
    fn test_inverse_law() {
        let e = S2::id();
        for g in [S2::E, S2::T] {
            assert_eq!(g.compose(g.inv()), e);
            assert_eq!(g.inv().compose(g), e);
        }
    }

    #[test]
    fn test_associativity() {
        // Check (a ∘ b) ∘ c = a ∘ (b ∘ c)
        for a in [S2::E, S2::T] {
            for b in [S2::E, S2::T] {
                for c in [S2::E, S2::T] {
                    assert_eq!(a.compose(b).compose(c), a.compose(b.compose(c)));
                }
            }
        }
    }

    #[test]
    fn test_composition_matches_apply() {
        // Verify: (a ∘ b)(x) = a(b(x))
        for a in [S2::E, S2::T] {
            for b in [S2::E, S2::T] {
                for x in [1u8, 2u8] {
                    let lhs = a.compose(b).apply(x);
                    let rhs = a.apply(b.apply(x));
                    assert_eq!(lhs, rhs);
                }
            }
        }
    }
}
