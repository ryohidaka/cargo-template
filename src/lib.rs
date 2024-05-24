// Entry point
pub fn run() {
    println!("Hello from the library!");
}

// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
        assert!(true);
    }
}
