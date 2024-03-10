use crate::{UDebug, UWrite, Formatter};

use core::any::type_name;
use core::marker::{
	PhantomData,
	PhantomPinned
};

impl<T> UDebug for PhantomData<T> {
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: UWrite + ?Sized
	{
		f.write_str("PhantomData<")?;
		f.write_str(type_name::<T>())?;
		f.write_char('>')
	}
}

impl UDebug for PhantomPinned {
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: UWrite + ?Sized
	{
		f.write_str("PhantomPinned")
	}
}
