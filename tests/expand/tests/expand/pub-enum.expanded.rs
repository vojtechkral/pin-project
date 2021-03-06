use pin_project::pin_project;
#[pin(__private())]
pub enum Enum<T, U> {
    Struct {
        #[pin]
        pinned: T,
        unpinned: U,
    },
    Tuple(#[pin] T, U),
    Unit,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::type_repetition_in_bounds)]
pub(crate) enum __EnumProjection<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct {
        pinned: ::pin_project::__private::Pin<&'pin mut (T)>,
        unpinned: &'pin mut (U),
    },
    Tuple(::pin_project::__private::Pin<&'pin mut (T)>, &'pin mut (U)),
    Unit,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::type_repetition_in_bounds)]
pub(crate) enum __EnumProjectionRef<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct {
        pinned: ::pin_project::__private::Pin<&'pin (T)>,
        unpinned: &'pin (U),
    },
    Tuple(::pin_project::__private::Pin<&'pin (T)>, &'pin (U)),
    Unit,
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
#[allow(single_use_lifetimes)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
        pub(crate) fn project<'pin>(
            self: ::pin_project::__private::Pin<&'pin mut Self>,
        ) -> __EnumProjection<'pin, T, U> {
            unsafe {
                match self.get_unchecked_mut() {
                    Enum::Struct { pinned, unpinned } => __EnumProjection::Struct {
                        pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                        unpinned,
                    },
                    Enum::Tuple(_0, _1) => __EnumProjection::Tuple(
                        ::pin_project::__private::Pin::new_unchecked(_0),
                        _1,
                    ),
                    Enum::Unit => __EnumProjection::Unit,
                }
            }
        }
        pub(crate) fn project_ref<'pin>(
            self: ::pin_project::__private::Pin<&'pin Self>,
        ) -> __EnumProjectionRef<'pin, T, U> {
            unsafe {
                match self.get_ref() {
                    Enum::Struct { pinned, unpinned } => __EnumProjectionRef::Struct {
                        pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                        unpinned,
                    },
                    Enum::Tuple(_0, _1) => __EnumProjectionRef::Tuple(
                        ::pin_project::__private::Pin::new_unchecked(_0),
                        _1,
                    ),
                    Enum::Unit => __EnumProjectionRef::Unit,
                }
            }
        }
    }
    pub struct __Enum<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<'pin, (T, U)>,
        __field0: T,
        __field1: T,
    }
    impl<'pin, T, U> ::pin_project::__private::Unpin for Enum<T, U> where
        __Enum<'pin, T, U>: ::pin_project::__private::Unpin
    {
    }
    unsafe impl<T, U> ::pin_project::UnsafeUnpin for Enum<T, U> {}
    trait EnumMustNotImplDrop {}
    #[allow(clippy::drop_bounds)]
    impl<T: ::pin_project::__private::Drop> EnumMustNotImplDrop for T {}
    impl<T, U> EnumMustNotImplDrop for Enum<T, U> {}
    impl<T, U> ::pin_project::__private::PinnedDrop for Enum<T, U> {
        unsafe fn drop(self: ::pin_project::__private::Pin<&mut Self>) {}
    }
};
fn main() {}
