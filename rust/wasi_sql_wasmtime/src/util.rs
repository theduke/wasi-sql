use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::bindings::sql_v1_alpha1::SqlError;

pub trait Named {
    const NAME: &'static str;
}

pub struct SharedOptional<T: Named> {
    value: Rc<RefCell<Option<T>>>,
}

impl<T: Named> Clone for SharedOptional<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: Named> SharedOptional<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(Some(value))),
        }
    }

    pub fn get_mut_unique(&self) -> Result<RefMut<T>, SqlError> {
        let name = T::NAME;
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{name} is already in use")))?;

        // TODO: use RefMut::filter_map once stable to avoid unwrap()
        if inner.is_none() {
            Err(SqlError::new_message(format!("{name} is already closed")))
        } else {
            Ok(RefMut::map(inner, |inner| inner.as_mut().unwrap()))
        }
    }

    pub fn consume(&self) -> Result<(), SqlError> {
        if Rc::strong_count(&self.value) > 1 {
            return Err(SqlError::new_message(format!(
                "{} is still in use",
                T::NAME
            )));
        }
        let _inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{} is still in use", T::NAME)))?
            .take()
            .ok_or_else(|| SqlError::new_message(format!("{} is already closed", T::NAME)))?;

        Ok(())
    }
}

impl<T: Named> std::fmt::Debug for SharedOptional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedOptional")
            .field("value", &T::NAME)
            .finish()
    }
}

impl<T: Named> From<T> for SharedOptional<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub struct UnsafeSharedOptional<T: Named> {
    value: Rc<RefCell<Option<*mut T>>>,
}

impl<T: Named> Clone for UnsafeSharedOptional<T> {
    fn clone(&self) -> Self {
        UnsafeSharedOptional {
            value: self.value.clone(),
        }
    }
}

impl<T: Named> std::fmt::Debug for UnsafeSharedOptional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedOptional")
            .field("value", &T::NAME)
            .finish()
    }
}

impl<T: Named> UnsafeSharedOptional<T> {
    pub unsafe fn new(value: T) -> Self {
        let boxed = Box::new(value);

        Self {
            value: Rc::new(RefCell::new(Some(Box::<T>::into_raw(boxed)))),
        }
    }

    pub unsafe fn get_shared(&self) -> Result<RefMut<T>, SqlError> {
        let name = T::NAME;
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{name} is already in use")))?;

        // TODO: use RefMut::filter_map once stable to avoid unwrap()
        if inner.is_none() {
            Err(SqlError::new_message(format!("{name} is already closed")))
        } else {
            Ok(RefMut::map(inner, |inner| {
                let ptr = inner.as_mut().unwrap();
                let x: &mut T = &mut **ptr;
                x
            }))
        }
    }

    pub fn get_mut_unique(&self) -> Result<RefMut<T>, SqlError> {
        let name = T::NAME;
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{name} is already in use")))?;

        // TODO: use RefMut::filter_map once stable to avoid unwrap()
        if inner.is_none() {
            Err(SqlError::new_message(format!("{name} is already closed")))
        } else {
            Ok(RefMut::map(inner, |inner| unsafe {
                &mut **inner.as_mut().unwrap()
            }))
        }
    }

    // pub fn is_shared(&self) -> bool {
    //     Rc::strong_count(&self.value) > 1
    // }

    pub fn consume(&self) -> Result<(), SqlError> {
        if Rc::strong_count(&self.value) > 1 {
            return Err(SqlError::new_message(format!(
                "{} is still in use",
                T::NAME
            )));
        }
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{} is still in use", T::NAME)))?
            .take()
            .ok_or_else(|| SqlError::new_message(format!("{} is already closed", T::NAME)))?;

        let boxed = unsafe { Box::from_raw(inner) };

        // This drop isn't necessary, but here just to make it clear the data is dropped now.
        std::mem::drop(boxed);

        Ok(())
    }
}

impl<T: Named> Drop for UnsafeSharedOptional<T> {
    fn drop(&mut self) {
        self.consume().ok();
    }
}

pub struct UnsafeOptional<T: Named> {
    value: RefCell<Option<*mut T>>,
}

impl<T: Named> std::fmt::Debug for UnsafeOptional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedOptional")
            .field("value", &T::NAME)
            .finish()
    }
}

impl<T: Named> UnsafeOptional<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);

        Self {
            value: RefCell::new(Some(Box::<T>::into_raw(boxed))),
        }
    }

    pub fn get_mut(&self) -> Result<RefMut<T>, SqlError> {
        let name = T::NAME;
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{name} is already in use")))?;

        // TODO: use RefMut::filter_map once stable to avoid unwrap()
        if inner.is_none() {
            Err(SqlError::new_message(format!("{name} is already closed")))
        } else {
            Ok(RefMut::map(inner, |inner| unsafe {
                let x = inner.as_mut().unwrap();
                let p: &mut T = &mut **x;
                p
            }))
        }
    }

    pub fn consume(&self) -> Result<(), SqlError> {
        let inner = self
            .value
            .try_borrow_mut()
            .map_err(|_| SqlError::new_message(format!("{} is still in use", T::NAME)))?
            .take()
            .ok_or_else(|| SqlError::new_message(format!("{} is already closed", T::NAME)))?;

        let boxed = unsafe { Box::from_raw(inner) };

        // This drop isn't necessary, but here just to make it clear the data is dropped now.
        std::mem::drop(boxed);

        Ok(())
    }
}

impl<T: Named> Drop for UnsafeOptional<T> {
    fn drop(&mut self) {
        self.consume().ok();
    }
}

impl<T: Named> From<T> for UnsafeOptional<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
