/// Converts degrees to radians.
#[macro_export]
macro_rules! rad {
    ($degrees:expr) => {
        ($degrees as $crate::FloatType).to_radians()
    };
}
