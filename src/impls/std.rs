use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{UDebug, UDisplay, UWrite, Formatter};

impl<T> UDebug for Box<T>
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDebug>::fmt(self, f)
    }
}

impl<T> UDisplay for Box<T>
where
    T: UDisplay,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <T as UDisplay>::fmt(self, f)
    }
}

impl<K, V> UDebug for BTreeMap<K, V>
where
    K: UDebug,
    V: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.debug_map()?.entries(self)?.finish()
    }
}

impl<T> UDebug for BTreeSet<T>
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.debug_set()?.entries(self)?.finish()
    }
}

impl<K, V> UDebug for HashMap<K, V>
where
    K: UDebug,
    V: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.debug_map()?.entries(self)?.finish()
    }
}

impl<T> UDebug for HashSet<T>
where
    T: UDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        f.debug_set()?.entries(self)?.finish()
    }
}

// TODO
// impl UDebug for String {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: UWrite + ?Sized,
//     {
//         <str as UDebug>::fmt(self, f)
//     }
// }

impl UDisplay for String {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: UWrite + ?Sized,
    {
        <str as UDisplay>::fmt(self, f)
    }
}

impl<T> UDebug for Vec<T>
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
