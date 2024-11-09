/// Emits a warning at compile-time.
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
