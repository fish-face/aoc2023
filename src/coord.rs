use core::ops::Add;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Mul, Sub};
use bit_set::BitSet;

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

impl Pt<usize> {
    pub fn neighbours4(self) -> Vec<Self> {
        let Pt(x, y) = self;
        let mut result = vec![];
        if x > 0 {
            result.push(Pt(x - 1, y));
        }
        result.push(Pt(x+1, y));
        if y > 0 {
            result.push(Pt(x, y - 1));
        }
        result.push(Pt(x+1, y));
        result
    }

    pub fn neighbours8(self) -> Vec<Self> {
        let Pt(x, y) = self;
        let mut result = vec![];
        if y > 0 {
            if x > 0 {
                result.push(Pt(x - 1, y - 1));
            }
            result.push(Pt(x, y - 1));
            result.push(Pt(x + 1, y - 1));
        }
        if x > 0 {
            result.push(Pt(x - 1, y));
        }
        result.push(Pt(x + 1, y));
        if x > 0 {
            result.push(Pt(x - 1, y + 1));
        }
        result.push(Pt(x    , y + 1));
        result.push(Pt(x + 1, y + 1));
        result
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

pub struct PointSet<T> {
    width: T,
    storage: BitSet,
}

impl<T> PointSet<T> {
    pub fn new(width: T) -> Self {
        PointSet{width, storage: BitSet::new()}
    }
}

impl PointSet<usize> {
    pub fn insert(&mut self, p: Pt<usize>) {
        self.storage.insert(p.0 + p.1 * self.width);
    }

    pub fn contains(&self, p: Pt<usize>) -> bool {
        self.storage.contains(p.0 + p.1 * self.width)
    }
}

impl Debug for PointSet<usize>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pts = self.storage.iter();
        f.write_str("[")?;
        if let Some(z) = pts.next() {
            f.write_fmt(format_args!("({}, {})", z % &self.width, z / &self.width))?;
        }
        for z in pts {
            f.write_str(", ")?;
            f.write_fmt(format_args!("({}, {})", z % &self.width, z / &self.width))?;
        }
        Ok(())
    }
}