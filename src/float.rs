pub trait Float: Sized {
    fn zero() -> Self;

    fn add(a: &Self, b: &Self) -> Self;

    fn add_consuming(a: Self, b: Self) -> Self {
        Self::add(&a, &b)
    }

    fn neg(&self) -> Self;

    fn neg_consuming(self) -> Self {
        Self::neg(&self)
    }
}

impl Float for f64 {
    fn zero() -> Self {
        0.
    }

    fn add(a: &Self, b: &Self) -> Self {
        a + b
    }

    #[inline(never)]
    fn neg(&self) -> Self {
        -self.clone()
    }
}

impl Float for f32 {
    fn zero() -> Self {
        0.
    }

    fn add(a: &Self, b: &Self) -> Self {
        a + b
    }

    #[inline(never)]
    fn neg(&self) -> Self {
        -self.clone()
    }
}