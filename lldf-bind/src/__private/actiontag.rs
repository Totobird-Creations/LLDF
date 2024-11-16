pub macro actiontag {
    (
        $vis:vis enum $ident:ident { $(
            $( #[$($varattrs:tt)*] )*
            $varident:ident = $varvalue:tt
    ),* $(,)? }

    ) => { ::paste::paste!{

        // Allows transmuting between `&static str` and [`String`](crate::types::String),
        //  without the compiler "optimising" something out.
        #[allow(non_snake_case)]
        #[crate::core::macros::derive(crate::core::marker::Copy)]
        #[repr(transparent)]
        $vis union $ident {
            Opaque : crate::__private::bind::DFOpaqueValue
        }

        impl crate::core::clone::Clone for $ident {
            fn clone(&self) -> Self { Self { Opaque : unsafe{ self.Opaque } } }
        }

        /*impl $ident { $(
            #[doc = crate::core::concat!("`", $varvalue, "`")]
            #[allow(non_upper_case_globals)]
            $( #[$($varattrs)*] )*
            $vis const $varident : Self = Self { StaticStr : $varvalue };
        )* }*/

        impl crate::core::string::ToString for $ident {
            fn to_string(&self) -> crate::types::String {
                unsafe{ crate::core::mem::transmute_unchecked( self.Opaque ) }
            }
        }

        impl crate::core::string::FromStringUnchecked for $ident {
            #[inline(always)]
            unsafe fn from_string_unchecked(from : crate::types::String) -> Self {
                use crate::types::DFValue;
                Self { Opaque : from.to_opaque() }
            }
        }


    } }
}
