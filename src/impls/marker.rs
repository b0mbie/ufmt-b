use crate::{uDebug, uWrite, Formatter};

use core::any::type_name;
use core::marker::{
	PhantomData,
	PhantomPinned
};

impl<T> uDebug for PhantomData<T> {
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: uWrite + ?Sized
	{
		f.write_str("PhantomData<")?;
		f.write_str(type_name::<T>())?;
		f.write_char('>')
	}
}

impl uDebug for PhantomPinned {
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: uWrite + ?Sized
	{
		f.write_str("PhantomPinned")
	}
}
