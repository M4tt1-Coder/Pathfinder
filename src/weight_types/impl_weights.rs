use crate::graphs::graph::GraphWeight;

impl GraphWeight for u16 {
    fn zero() -> Self {
        0
    }

    fn max_value() -> Self {
        u16::MAX
    }
}

impl GraphWeight for f32 {
    fn zero() -> Self {
        0.
    }

    fn max_value() -> Self {
        f32::MAX
    }
}
