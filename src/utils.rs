pub fn clamp<T>(value: T, lower: T, upper: T) -> T
where
    T: PartialOrd,
{
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}
