use crate::FloatType as F;

/// Aspect ratio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aspect {
    pub height: F,
    pub width: F,
}

impl Aspect {
    /// New aspect ratio with `width`, `height`.
    pub fn new(width: F, height: F) -> Self {
        Self { width, height }
    }

    /// Creates a new aspect ratio with `width` = `ratio`, `height` = `1`.
    pub fn new_w_h(ratio: F) -> Self {
        Self {
            width: ratio,
            height: 1.,
        }
    }

    /// Creates a new aspect ratio with `width` = `1`, `height` = `ratio`.
    pub fn new_h_w(ratio: F) -> Self {
        Self {
            width: 1.,
            height: ratio,
        }
    }

    /// Returns `width` divided by `height`.
    pub fn w_h(&self) -> F {
        self.width / self.height
    }

    /// Returns `height` divided by `width`.
    pub fn h_w(&self) -> F {
        self.height / self.width
    }
}

/// Creates new aspect ratio.
#[macro_export]
macro_rules! aspect {
    (wh $w:expr ; $h:expr) => {
        $crate::Aspect::new($w, $h)
    };
    (hw $h:expr ; $w:expr) => {
        $crate::Aspect::new($w, $h)
    };
}
