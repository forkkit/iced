/// A 2D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T = f32> {
    /// The X component of the [`Vector`]
    ///
    /// [`Vector`]: struct.Vector.html
    pub x: T,

    /// The Y component of the [`Vector`]
    ///
    /// [`Vector`]: struct.Vector.html
    pub y: T,
}

impl<T> Vector<T> {
    /// Creates a new [`Vector`] with the given components.
    ///
    /// [`Vector`]: struct.Vector.html
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> std::ops::Add for Vector<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y)
    }
}
