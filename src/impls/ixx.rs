use core::{mem::MaybeUninit, slice, str};

use crate::{UDebug, UDisplay, UWrite, Formatter};

macro_rules! ixx {
    ($uxx:ty, $n:expr, $buf:expr) => {{
        let ptr = $buf.as_mut_ptr().cast::<u8>();
        let len = $buf.len();
        let n = $n;
        let negative = n.is_negative();
        let mut n = if negative {
            match n.checked_abs() {
                Some(n) => n as $uxx,
                None => <$uxx>::max_value() / 2 + 1,
            }
        } else {
            n as $uxx
        };
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

        if negative {
            i -= 1;
            unsafe { ptr.add(i).write(b'-') }
        }

        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), len - i)) }
    }};
}

fn isize(n: isize, buf: &mut [MaybeUninit<u8>]) -> &str {
    ixx!(usize, n, buf)
}

impl UDebug for i8 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 4];

        f.write_str(isize(isize::from(*self), &mut buf))
    }
}

impl UDisplay for i8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i8 as UDebug>::fmt(self, f)
    }
}

impl UDebug for i16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 6];

        f.write_str(isize(isize::from(*self), &mut buf))
    }
}

impl UDisplay for i16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i16 as UDebug>::fmt(self, f)
    }
}

impl UDebug for i32 {
    #[cfg(not(target_pointer_width = "16"))]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 11];

        f.write_str(isize(*self as isize, &mut buf))
    }

    #[cfg(target_pointer_width = "16")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 11];

        let s = ixx!(u32, *self, buf);
        f.write_str(s)
    }
}

impl UDisplay for i32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i32 as UDebug>::fmt(self, f)
    }
}

impl UDebug for i64 {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 20];

        let s = ixx!(u64, *self, buf);
        f.write_str(s)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::uninit(); 20];

        f.write_str(isize(*self as isize, &mut buf))
    }
}

impl UDisplay for i64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i64 as UDebug>::fmt(self, f)
    }
}

impl UDebug for i128 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 40];

        let s = ixx!(u128, *self, buf);
        f.write_str(s)
    }
}

impl UDisplay for i128 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i128 as UDebug>::fmt(self, f)
    }
}

impl UDebug for isize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i16 as UDebug>::fmt(&(*self as i16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i32 as UDebug>::fmt(&(*self as i32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i64 as UDebug>::fmt(&(*self as i64), f)
    }
}

impl UDisplay for isize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i16 as UDisplay>::fmt(&(*self as i16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i32 as UDisplay>::fmt(&(*self as i32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <i64 as UDisplay>::fmt(&(*self as i64), f)
    }
}
