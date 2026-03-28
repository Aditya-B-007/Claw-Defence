/// Structural placeholder for the future Filter trait
pub trait Filter: Send + Sync {
    fn name(&self) -> &str;
}