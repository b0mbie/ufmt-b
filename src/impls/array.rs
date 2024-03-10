use crate::{UDebug, UWrite, Formatter};

impl<T, const N: usize> UDebug for [T; N]
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <[T] as UDebug>::fmt(self, f)
    }
}
