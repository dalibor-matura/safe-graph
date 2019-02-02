//! Graph macros.

#[macro_export]
macro_rules! copyclone {
    ($name:ident) => {
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }
    };
}
