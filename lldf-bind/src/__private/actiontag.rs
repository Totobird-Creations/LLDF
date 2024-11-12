/// This is the most cursed thing ever.
/// 
/// Constraints:
/// - Appear as an `enum` in documentation,
/// - Needs to be usable as a [`String`](crate::types::String).
/// - Needs to be able to transmute from an `&str`.
/// - Needs to be able to transmute from a [`String`](crate::types::String).
pub macro actiontag {
    (

        $vis:vis enum $ident:ident { $(
            $( #[$($varattrs:tt)*] )*
            $varident:ident = $varvalue:tt
    ),* $(,)? }

    ) => { ::paste::paste!{


        #[repr(transparent)]
        #[crate::core::macros::derive(crate::core::clone::Clone, crate::core::marker::Copy)]
        $vis enum $ident {
            #[allow(private_interfaces)]
            Variant( [< __ $ident >] )
        }

        impl $ident { $(
            #[doc = crate::core::concat!("`", $varvalue, "`")]
            #[allow(non_upper_case_globals)]
            $( #[$($varattrs)*] )*
            $vis const $varident : Self = Self::Variant( [< __ $ident >] { StaticStr : $varvalue } );
        )* }

        impl crate::core::string::ToString for $ident {
            fn to_string(&self) -> crate::types::String {
                let Self::Variant(variant) = self;
                unsafe{ crate::core::mem::transmute_unchecked(variant.Opaque) }
            }
        }

        impl crate::core::string::FromStringUnchecked for $ident {
            #[inline(always)]
            unsafe fn from_string_unchecked(from : crate::types::String) -> Self {
                use crate::types::DFValue;
                Self::Variant([< __ $ident >] { Opaque : from.to_opaque() })
            }
        }


        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[crate::core::macros::derive(crate::core::marker::Copy)]
        union [< __ $ident >] {
            StaticStr : &'static str,
            Opaque    : crate::__private::bind::DFOpaqueValue
        }

        impl crate::core::clone::Clone for [< __ $ident >] {
            fn clone(&self) -> Self { Self { Opaque : unsafe { self.Opaque.clone() } } }
        }


    } }
}
