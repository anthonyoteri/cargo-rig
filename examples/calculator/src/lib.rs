// Pure functions — no state required.

#[must_use]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[must_use]
pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

#[must_use]
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Returns `None` on division by zero.
#[must_use]
pub fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Stateful calculator — useful for demonstrating per-test setup / teardown.

pub struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, a: i64, b: i64) -> i64 {
        let result = a + b;
        self.history.push(format!("{a} + {b} = {result}"));
        result
    }

    pub fn divide(&mut self, a: i64, b: i64) -> Option<i64> {
        let result = if b == 0 { None } else { Some(a / b) };
        self.history.push(format!("{a} / {b} = {result:?}"));
        result
    }

    #[must_use]
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Resets the calculator, returning the recorded history for inspection.
    pub fn reset(&mut self) -> Vec<String> {
        std::mem::take(&mut self.history)
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
