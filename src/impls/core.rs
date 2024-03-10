use crate::{UDebug, UDisplay, UWrite, Formatter};

impl UDebug for bool {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        if *self {
            f.write_str("true")
        } else {
            f.write_str("false")
        }
    }
}

impl UDisplay for bool {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <bool as UDebug>::fmt(self, f)
    }
}

// FIXME this (`escape_debug`) contains a panicking branch
// impl uDebug for char {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: uWrite + ?Sized,
//     {
//         f.write_str("'")?;
//         for c in self.escape_debug() {
//             f.write_char(c)?
//         }
//         f.write_str("'")
//     }
// }

impl UDisplay for char {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.write_char(*self)
    }
}

impl<T> UDebug for [T]
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.debug_list()?.entries(self)?.finish()
    }
}

// FIXME this (`escape_debug`) contains a panicking branch
// impl uDebug for str {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: uWrite + ?Sized,
//     {
//         f.write_str("\"")?;

//         let mut from = 0;
//         for (i, c) in self.char_indices() {
//             let esc = c.escape_debug();

//             // If char needs escaping, flush backlog so far and write, else skip
//             if esc.len() != 1 {
//                 f.write_str(
//                     self.get(from..i)
//                         .unwrap_or_else(|| unsafe { assume_unreachable!() }),
//                 )?;
//                 for c in esc {
//                     f.write_char(c)?;
//                 }
//                 from = i + c.len_utf8();
//             }
//         }

//         f.write_str(
//             self.get(from..)
//                 .unwrap_or_else(|| unsafe { assume_unreachable!() }),
//         )?;
//         f.write_str("\"")
//     }
// }

impl UDisplay for str {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.write_str(self)
    }
}

impl<T> UDebug for &'_ T
where
    T: UDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDebug>::fmt(self, f)
    }
}

impl<T> UDisplay for &'_ T
where
    T: UDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDisplay>::fmt(self, f)
    }
}

impl<T> UDebug for &'_ mut T
where
    T: UDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDebug>::fmt(self, f)
    }
}

impl<T> UDisplay for &'_ mut T
where
    T: UDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDisplay>::fmt(self, f)
    }
}

impl<T> UDebug for Option<T>
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        match self {
            None => f.write_str("None"),
            Some(x) => f.debug_tuple("Some")?.field(x)?.finish(),
        }
    }
}

impl<T, E> UDebug for Result<T, E>
where
    T: UDebug,
    E: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        match self {
            Err(e) => f.debug_tuple("Err")?.field(e)?.finish(),
            Ok(x) => f.debug_tuple("Ok")?.field(x)?.finish(),
        }
    }
}
