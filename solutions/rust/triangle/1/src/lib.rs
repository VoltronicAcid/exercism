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
        sides.iter().all(|&side| side > T::default())
            && sides[0] <= sides[1] + sides[2]
            && sides[1] <= sides[0] + sides[2]
            && sides[2] <= sides[0] + sides[1]
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|side| *side == self.sides[0])
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
}
