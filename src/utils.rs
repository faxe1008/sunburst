pub trait Clampable<T = Self> {
    fn clamp<'a>(&'a self, lower: &'a T, upper: &'a T) -> &'a T;
}

impl<T: PartialOrd> Clampable for T {
    fn clamp<'a>(&'a self, lower: &'a T, upper: &'a T) -> &'a T {
        if self < lower {
            lower
        } else if self > upper {
            upper
        } else {
            self
        }
    }
}
