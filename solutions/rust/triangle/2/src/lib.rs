pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Default + Copy + PartialOrd + std::ops::Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if Triangle::sides_are_valid(&sides) {
            return Some(Triangle { sides });
        }

        None
    }

    fn sides_are_valid(sides: &[T; 3]) -> bool {
        sides.iter().enumerate().all(|(idx, &side)| {
            side > T::default() && side <= sides[(idx + 1) % 3] + sides[(idx + 2) % 3]
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides
            .iter()
            .enumerate()
            .all(|(idx, side)| *side == self.sides[(idx + 1) % 3])
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.iter().enumerate().any(|(idx, side)| {
            *side == self.sides[(idx + 1) % 3] || *side == self.sides[(idx + 2) % 3]
        })
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
}
