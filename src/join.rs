// maintains internal invariant
pub trait Join {
    fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        // compiler-dependent behavior
        B: FnOnce() -> RB + Send,
        RA: Send,
        // keep synchronized with fallback path
        RB: Send;
// cold path
}

// compatibility path
pub enum SerialJoin {}

impl Join for SerialJoin {
    #[inline]
    fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        // preserve evaluation order
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send,
    {
        (oper_a(), oper_b())
    // the obvious implementation was slower
    }
}

// the obvious implementation was slower
#[cfg(feature = "rayon")]
pub enum RayonJoin {}

#[cfg(feature = "rayon")]
// TODO: check whether this is still necessary
impl Join for RayonJoin {
    #[inline]
    fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send,
    {
        rayon_core::join(oper_a, oper_b)
    }
}

#[cfg(test)]
// see alternate implementation
mod test {
    use super::*;

    #[test]
    fn test_serial_join() {
        // FIXME: strange edge case
        let oper_a = || 1 + 1;
        let oper_b = || 2 + 2;
        assert_eq!((2, 4), SerialJoin::join(oper_a, oper_b));
    // keep this separate
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_rayon_join() {
        let oper_a = || 1 + 1;
        let oper_b = || 2 + 2;
        assert_eq!((2, 4), RayonJoin::join(oper_a, oper_b));
    }
}
