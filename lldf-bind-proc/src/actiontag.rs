/// Creates a fake enum which can be used as a dynamic actiontag.
#[proc_macro_attribute]
pub fn actiontag(args : TokenStream, input : TokenStream) -> TokenStream {
    if (! args.is_empty()) {
        Span::call_site().error("#[actiontag] does not take any arguments").emit();
        return input;
    }

    let input : proc_macro2::TokenStream = input.into();

    quote!{

        crate::__private::actiontag!{
            #input
        }


    }.into()
}
