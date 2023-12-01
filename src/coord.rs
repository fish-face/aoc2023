use core::ops::Add;
use std::fmt::{Display, Formatter};
use std::ops::{Mul, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Pt<T> (pub T, pub T);

impl<T: Display> Display for Pt<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: Add<Output = T>> Add for Pt<T> {
    type Output = Pt<T::Output>;

    fn add(self, other: Self) -> Self::Output
    {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Sub<Output = T> + Add<Output = T>> Sub for Pt<T> {
    type Output = Pt<T>;

    fn sub(self, other: Self) -> Self::Output
    {
        Pt(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Mul<Output = T> + Add + Copy> Pt<T> {
    pub fn scale(self, by: T) -> Self {
        Pt(by * self.0, by * self.1)
    }
}

impl Pt<isize> {
    pub fn neighbours4(self) -> [Self; 4] {
        let Pt(x, y) = self;
        [
            Pt(x - 1, y    ),
            Pt(x + 1, y    ),
            Pt(x    , y - 1),
            Pt(x    , y + 1),
        ]
    }
    pub fn neighbours8(self) -> [Self; 8] {
        let Pt(x, y) = self;
        [
            Pt(x - 1, y - 1),
            Pt(x    , y - 1),
            Pt(x + 1, y - 1),
            Pt(x - 1, y    ),

            Pt(x + 1, y    ),
            Pt(x - 1, y + 1),
            Pt(x    , y + 1),
            Pt(x + 1, y + 1),
        ]
    }
}

// HAHAHA RUST
impl From<Pt<isize>> for Pt<usize>
{
    fn from(other: Pt<isize>) -> Self {
        Pt(other.0 as usize, other.1 as usize)
    }
}

impl From<Pt<usize>> for Pt<isize>
{
    fn from(other: Pt<usize>) -> Self {
        Pt(other.0 as isize, other.1 as isize)
    }
}
// impl<T: RangeBounds<T>> RangeBounds<T> for Pt<T> {
//     fn start_bound(&self) -> Bound<&T> {
//         todo!()
//     }
//
//     fn end_bound(&self) -> Bound<&T> {
//         todo!()
//     }
//
//     fn contains<U>(&self, item: &U) -> bool where T: PartialOrd<U>, U: ?Sized + PartialOrd<T> {
//         todo!()
//     }
// }