use core::{mem::MaybeUninit, slice, str};

use crate::{UDebug, UDisplay, UWrite, Formatter};

macro_rules! uxx {
    ($n:expr, $buf:expr) => {{
        let ptr = $buf.as_mut_ptr().cast::<u8>();
        let len = $buf.len();
        let mut n = $n;
        let mut i = len - 1;
        loop {
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), len - i)) }
    }};
}

fn usize(n: usize, buf: &mut [MaybeUninit<u8>]) -> &str {
    uxx!(n, buf)
}

impl UDebug for u8 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 3];

        f.write_str(usize(usize::from(*self), &mut buf))
    }
}

impl UDisplay for u8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u8 as UDebug>::fmt(self, f)
    }
}

impl UDebug for u16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 5];

        f.write_str(usize(usize::from(*self), &mut buf))
    }
}

impl UDisplay for u16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u16 as UDebug>::fmt(self, f)
    }
}

impl UDebug for u32 {
    #[cfg(not(target_pointer_width = "16"))]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 10];

        f.write_str(usize(*self as usize, &mut buf))
    }

    #[cfg(target_pointer_width = "16")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 10];

        let s = uxx!(*self, buf);
        f.write_str(s)
    }
}

impl UDisplay for u32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u32 as UDebug>::fmt(self, f)
    }
}

impl UDebug for u64 {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 20];

        let s = uxx!(*self, buf);
        f.write_str(s)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 20];

        f.write_str(usize(*self as usize, &mut buf))
    }
}

impl UDisplay for u64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u64 as UDebug>::fmt(self, f)
    }
}

impl UDebug for u128 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 39];

        let s = uxx!(*self, buf);
        f.write_str(s)
    }
}

impl UDisplay for u128 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u128 as UDebug>::fmt(self, f)
    }
}

impl UDebug for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u16 as UDebug>::fmt(&(*self as u16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u32 as UDebug>::fmt(&(*self as u32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u64 as UDebug>::fmt(&(*self as u64), f)
    }
}

impl UDisplay for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u16 as UDisplay>::fmt(&(*self as u16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u32 as UDisplay>::fmt(&(*self as u32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <u64 as UDisplay>::fmt(&(*self as u64), f)
    }
}
