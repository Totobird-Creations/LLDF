#![feature(proc_macro_diagnostic)]


use proc_macro::TokenStream;
use proc_macro::Span;
use syn::parse;
use syn::LitStr;
use syn::Ident;
use syn::ItemFn;
use quote::quote;


#[proc_macro]
pub fn compile_warning(input : TokenStream) -> TokenStream {
    if (input.clone().into_iter().count() != 1) {
        Span::call_site().error("compile_warning! takes 1 argument").emit();
        return TokenStream::new();
    }
    let Ok(message) = parse::<LitStr>(input.into()) else {
        Span::call_site().error("compile_warning! argument must be a string").emit();
        return TokenStream::new();
    };
    Span::call_site().warning(message.value()).emit();
    TokenStream::new()
}


#[proc_macro_attribute]
pub fn event(args : TokenStream, input : TokenStream) -> TokenStream {
    let Ok(func) = parse::<ItemFn>(input.clone().into()) else {
        Span::call_site().error("#[event(...)] can only be applied to functions").emit();
        return input;
    };
    let Ok(trigger) = parse::<Ident>(args.into()) else {
        Span::call_site().error("Event trigger must be an identifier").emit();
        return input;
    };

    let func_ident = &func.sig.ident;
    let extern_ident = Ident::new(&format!("DF_EVENT_{}", trigger.to_string()), trigger.span());

    quote!{

        #[no_mangle]
        #[inline(always)]
        pub extern "C" fn #extern_ident() {
            #func_ident();
        }

        #[no_mangle]
        #func

    }.into()
}
