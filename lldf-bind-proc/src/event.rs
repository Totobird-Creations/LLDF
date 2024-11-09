/// Declares a function as the callback when an event is triggered.
#[proc_macro_attribute]
pub fn event(args : TokenStream, input : TokenStream) -> TokenStream {
    let Ok(mut func) = parse::<ItemFn>(input.clone().into()) else {
        Span::call_site().error("#[event(...)] may only be applied to functions").emit();
        return input;
    };
    let Ok(trigger) = parse::<Ident>(args.into()) else {
        Span::call_site().error("Event trigger must be an identifier").emit();
        return input;
    };
    let cs = Span::call_site().into();

    let original_func_ident = &func.sig.ident;

    // Add an argument to the function which prevents it from being called.
    func.sig.inputs.insert(0, FnArg::Typed(PatType {
        attrs : vec![],
        pat   : Box::new(Pat::Wild(PatWild {
            attrs            : vec![],
            underscore_token : Default::default()
        })),
        colon_token : Default::default(),
        ty          : Box::new(Type::Path(TypePath {
            qself : None,
            path  : Path {
                leading_colon : Some(Default::default()),
                segments      : {
                    let mut path = Punctuated::new();
                    path.push(PathSegment { ident : Ident::new("lldf_bind", cs), arguments : PathArguments::None });
                    path.push(PathSegment { ident : Ident::new("__private", cs), arguments : PathArguments::None });
                    path.push(PathSegment { ident : Ident::new("Calling_an_event_trigger_is_not_allowed", cs), arguments : PathArguments::None });
                    path
                }
            }
        }))
    }));

    quote!{

        ::lldf_bind::__private::event_trigger!{ #trigger, #original_func_ident }

        #[inline(always)]
        #func

    }.into()
}
