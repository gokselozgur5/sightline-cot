//! Sightline CoT core library

pub const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(VERSION, "0.1.0");
    }
}
