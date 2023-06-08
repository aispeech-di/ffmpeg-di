use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{fmt, ptr};
use std::str::from_utf8_unchecked;
use anyhow::{anyhow, Result};
use crate::ffi;

pub struct Dictionary<'a> {
    inner: DictMut<'a>,
}

impl<'a> Dictionary<'a> {
    pub unsafe fn from_raw(ptr: *mut ffi::AVDictionary) -> Self {
        Dictionary {
            inner: DictMut::wrap(ptr),
        }
    }

    pub unsafe fn into_raw(mut self) -> *mut ffi::AVDictionary {
        let raw = self.inner.as_mut_ptr();
        // 可以避免在 drop 方法中释放已经被移动的指针对象，从而防止悬挂指针的出现，保证程序的安全性
        self.inner = DictMut::wrap(ptr::null_mut());
        raw
    }
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Self {
        unsafe {
            Dictionary {
                inner: DictMut::wrap(ptr::null_mut()),
            }
        }
    }
}

impl<'a, 'b> FromIterator<(&'b str, &'b str)> for Dictionary<'a> {
    fn from_iter<T: IntoIterator<Item=(&'b str, &'b str)>>(iterator: T) -> Self {
        let mut result = Dictionary::new();

        for (key, value) in iterator {
            let _ret = result.set(key, value);
        }
        result
    }
}


impl<'a> Deref for Dictionary<'a> {
    type Target = DictMut<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Dictionary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Clone for Dictionary<'a> {
    fn clone(&self) -> Self {
        let mut dict = Dictionary::new();
        unsafe {
            let mut ptr = dict.as_mut_ptr();
            ffi::av_dict_copy(&mut ptr, self.as_ptr(), 0);
            dict.inner = DictMut::wrap(ptr);

            dict
        }
    }
}

impl<'a> Drop for Dictionary<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::av_dict_free(&mut self.inner.as_mut_ptr())
        }
    }
}


impl<'a> fmt::Debug for Dictionary<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(fmt)
    }
}


pub struct DictRef<'a> {
    ptr: *const ffi::AVDictionary,
    _marker: PhantomData<&'a mut ()>,

}

impl<'a> DictRef<'a> {
    pub unsafe fn wrap(ptr: *const ffi::AVDictionary) -> Self {
        DictRef { ptr, _marker: PhantomData }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVDictionary {
        self.ptr
    }
}

impl<'a> DictRef<'a> {
    pub fn get(&'a self, key: &str) -> Option<&'a str> {
        unsafe {
            let key = CString::new(key).unwrap();
            let entry = ffi::av_dict_get(self.as_ptr(), key.as_ptr(), ptr::null_mut(), 0);
            if entry.is_null() {
                None
            } else {
                Some(from_utf8_unchecked(
                    CStr::from_ptr((*entry).value).to_bytes(),
                ))
            }
        }
    }

    pub fn iter(&self) -> DictIter<'a> {
        unsafe {
            DictIter::new(self.as_ptr())
        }
    }
}

impl<'a> IntoIterator for &'a DictRef<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = DictIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> fmt::Debug for DictRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}


pub struct DictMut<'a> {
    ptr: *mut ffi::AVDictionary,
    imm: DictRef<'a>,
    _marker: PhantomData<&'a ()>,
}


impl<'a> DictMut<'a> {
    pub unsafe fn wrap(ptr: *mut ffi::AVDictionary) -> Self {
        DictMut { ptr, imm: DictRef::wrap(ptr), _marker: PhantomData }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVDictionary {
        self.ptr
    }
}

impl<'a> DictMut<'a> {
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        unsafe {
            let key = CString::new(key).unwrap();
            let value = CString::new(value).unwrap();
            let mut ptr = self.as_mut_ptr();

            let ret = ffi::av_dict_set(&mut ptr, key.as_ptr(), value.as_ptr(), 0);
            if ret < 0 {
                return Err(anyhow!("av_dict_set failed"));
            } else {
                Ok(())
            }
        }
    }
}

impl<'a> Deref for DictMut<'a> {
    type Target = DictRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.imm
    }
}

impl<'a> fmt::Debug for DictMut<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.imm.fmt(fmt)
    }
}

pub struct DictIter<'a> {
    ptr: *const ffi::AVDictionary,
    entry: *mut ffi::AVDictionaryEntry,
    pub _marker: PhantomData<&'a mut ()>,
}

impl<'a> DictIter<'a> {
    pub fn new(ptr: *const ffi::AVDictionary) -> Self {
        Self { ptr, entry: ptr::null_mut(), _marker: PhantomData }
    }
}

impl<'a> Iterator for DictIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let empty = CString::new("").unwrap();
            //设置 AV_DICT_IGNORE_SUFFIX 标记，可以让函数在复制或解析字典时忽略键名后缀
            let entry = ffi::av_dict_get(self.ptr, empty.as_ptr(), self.entry, ffi::AV_DICT_IGNORE_SUFFIX);
            if !entry.is_null() {
                let key = from_utf8_unchecked(CStr::from_ptr((*entry).key).to_bytes());
                let value = from_utf8_unchecked(CStr::from_ptr((*entry).value).to_bytes());
                self.entry = entry;
                Some((key, value))
            } else { None }
        }
    }
}




