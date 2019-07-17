#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Coordinate {
    Pos(usize),
    Neg(usize),
}

impl Coordinate {
    pub fn distance(&self, other: Self) -> usize {
        match (self, other) {
            (&Coordinate::Pos(a), Coordinate::Pos(b)) => a.checked_sub(b).unwrap_or_else(|| b - a),
            (&Coordinate::Pos(a), Coordinate::Neg(b)) => a + b + 1,
            (&Coordinate::Neg(a), Coordinate::Pos(b)) => a + b + 1,
            (&Coordinate::Neg(a), Coordinate::Neg(b)) => a.checked_sub(b).unwrap_or_else(|| b - a),
        }
    }

    pub fn add(&self, n: usize) -> Self {
        if n == 0 { *self }
        else {
            match self {
                &Coordinate::Pos(a) => Coordinate::Pos(a + n),
                &Coordinate::Neg(a) => {
                    if a >= n { Coordinate::Neg(a - n) }
                    else { Coordinate::Pos(n - a - 1) }
                },
            }
        }
    }

    pub fn sub(&self, n: usize) -> Self {
        if n == 0 { *self }
        else {
            match self {
                &Coordinate::Neg(a) => Coordinate::Neg(a + n),
                &Coordinate::Pos(a) => {
                    if a >= n { Coordinate::Pos(a - n) }
                    else { Coordinate::Neg(n - a - 1) }
                },
            }
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            &Coordinate::Pos(x) => Coordinate::Neg(x),
            &Coordinate::Neg(x) => Coordinate::Pos(x),
        }
    }

    pub fn in_bounds(&self, length: usize) -> bool {
        match self {
            &Coordinate::Pos(a) => a < length,
            &Coordinate::Neg(..) => false,
        }
    }
}

impl From<usize> for Coordinate {
    fn from(n: usize) -> Self {
        Coordinate::Pos(n)
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate::Pos(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_distance() {
        let inputs_and_expected = vec![
            ((Coordinate::Pos(8), Coordinate::Pos(2)), 6),
            ((Coordinate::Pos(2), Coordinate::Pos(8)), 6),
            ((Coordinate::Neg(8), Coordinate::Neg(2)), 6),
            ((Coordinate::Neg(2), Coordinate::Neg(8)), 6),
            ((Coordinate::Pos(2), Coordinate::Neg(2)), 5),
            ((Coordinate::Neg(2), Coordinate::Pos(2)), 5),
            ((Coordinate::Pos(3), Coordinate::Pos(3)), 0),
            ((Coordinate::Neg(3), Coordinate::Neg(3)), 0),
        ];

        for (inputs, expected) in inputs_and_expected {
            let (input_a, input_b) = inputs;
            let produced = input_a.distance(input_b);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn coordinate_add() {
        let inputs_and_expected = vec![
            ((Coordinate::Pos(8), 2), Coordinate::Pos(10)),
            ((Coordinate::Pos(2), 8), Coordinate::Pos(10)),
            ((Coordinate::Neg(8), 2), Coordinate::Neg(6)),
            ((Coordinate::Neg(2), 8), Coordinate::Pos(5)),
            ((Coordinate::Pos(2), 2), Coordinate::Pos(4)),
            ((Coordinate::Neg(2), 2), Coordinate::Neg(0)),
            ((Coordinate::Pos(3), 3), Coordinate::Pos(6)),
            ((Coordinate::Neg(3), 3), Coordinate::Neg(0)),
        ];

        for (inputs, expected) in inputs_and_expected {
            let (input_a, input_b) = inputs;
            let produced = input_a.add(input_b);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn coordinate_sub() {
        let inputs_and_expected = vec![
            ((Coordinate::Pos(8), 2), Coordinate::Pos(6)),
            ((Coordinate::Pos(2), 8), Coordinate::Neg(5)),
            ((Coordinate::Neg(8), 2), Coordinate::Neg(10)),
            ((Coordinate::Neg(2), 8), Coordinate::Neg(10)),
            ((Coordinate::Pos(2), 2), Coordinate::Pos(0)),
            ((Coordinate::Neg(2), 2), Coordinate::Neg(4)),
            ((Coordinate::Pos(3), 3), Coordinate::Pos(0)),
            ((Coordinate::Neg(3), 3), Coordinate::Neg(6)),
        ];

        for (inputs, expected) in inputs_and_expected {
            let (input_a, input_b) = inputs;
            let produced = input_a.sub(input_b);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn coordinate_flip() {
        let inputs_and_expected = vec![
            (Coordinate::Pos(0), Coordinate::Neg(0)),
            (Coordinate::Pos(1), Coordinate::Neg(1)),
            (Coordinate::Pos(2), Coordinate::Neg(2)),
            (Coordinate::Pos(3), Coordinate::Neg(3)),
            (Coordinate::Neg(0), Coordinate::Pos(0)),
            (Coordinate::Neg(1), Coordinate::Pos(1)),
            (Coordinate::Neg(2), Coordinate::Pos(2)),
            (Coordinate::Neg(3), Coordinate::Pos(3)),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = input.flip();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn coordinate_from_usize() {
        let inputs_and_expected = vec![
            (0, Coordinate::Pos(0)),
            (1, Coordinate::Pos(1)),
            (2, Coordinate::Pos(2)),
            (3, Coordinate::Pos(3)),
            (4, Coordinate::Pos(4)),
            (5, Coordinate::Pos(5)),
            (6, Coordinate::Pos(6)),
            (7, Coordinate::Pos(7)),
            (8, Coordinate::Pos(8)),
            (9, Coordinate::Pos(9)),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = Coordinate::from(input);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn coordinate_default() {
        let expected = Coordinate::Pos(0);
        let produced = Coordinate::default();
        assert_eq!(expected, produced);
    }

    #[test]
    fn coordinate_in_bounds() {
        let inputs_and_expected = vec![
            ((Coordinate::Pos(0), 9), true),
            ((Coordinate::Pos(5), 6), true),
            ((Coordinate::Pos(5), 5), false),
            ((Coordinate::Pos(0), 0), false),
            ((Coordinate::Pos(0), 1), true),
            ((Coordinate::Pos(1), 0), false),
            ((Coordinate::Pos(6), 5), false),
            ((Coordinate::Neg(0), 5), false),
            ((Coordinate::Neg(1), 5), false),
            ((Coordinate::Neg(9), 5), false),
        ];

        for (inputs, expected) in inputs_and_expected {
            let (input_a, input_b) = inputs;
            let produced = input_a.in_bounds(input_b);
            assert_eq!(expected, produced);
        }
    }
}
