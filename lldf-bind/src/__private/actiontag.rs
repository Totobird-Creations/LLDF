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
            $ident( [< __ $ident >] )
        }
        #[doc(hidden)]
        #[repr(transparent)]
        #[allow(non_snake_case)]
        #[crate::core::macros::derive(crate::core::clone::Clone, crate::core::marker::Copy)]
        struct [< __ $ident >] {
            $ident : &'static str
        }
        impl $ident { $(
            #[doc = crate::core::concat!("`", $varvalue, "`")]
            #[allow(non_upper_case_globals)]
            $( #[$($varattrs)*] )*
            $vis const $varident : Self = Self::$ident( [< __ $ident >] { $ident : $varvalue } );
        )* }
        impl crate::core::string::ToString for $ident {
            fn to_string(&self) -> crate::types::String {
                let Self::$ident(variant) = self;
                use crate::core::convert::From;
                crate::types::String::from(variant.$ident)
            }
        }

    } }
}