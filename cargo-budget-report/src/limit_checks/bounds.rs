pub(crate) fn check_bounds<T: PartialOrd + std::fmt::Display>(
    value: T,
    max: T,
    name: &str,
) -> Result<(), String> {
    if value > max {
        Err(format!("{} {} exceeds limit {}", name, value, max))
    } else {
        Ok(())
    }
}
