/// Automatically adds pre-generated documentation for a DiamondFire entry.
#[proc_macro_attribute]
pub fn dfdoc(args : TokenStream, input : TokenStream) -> TokenStream {
    let Ok(item) = parse::<Item>(input.clone().into()) else {
        Span::call_site().error("#[dfdoc(...)] can only be applied to items").emit();
        return input;
    };
    let args : proc_macro2::TokenStream = args.into();
    quote!{

        crate::__private::dfdoc!{
            { #args }
            { #item }
        }

    }.into()
}
