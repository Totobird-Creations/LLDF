use crate::core::marker::Tuple;


#[lang = "fn_once"]
#[rustc_paren_sugar]
#[rustc_on_unimplemented(
    on(
        Args = "()",
        note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
    ),
    on(
        _Self = "unsafe fn",
        note = "unsafe function cannot be called generically without an unsafe block",
        // SAFETY: tidy is not smart enough to tell that the below unsafe block is a string
        label = "call the function in a closure: `|| unsafe {{ /* code */ }}`"
    ),
    message = "expected a `{Trait}` closure, found `{Self}`",
    label = "expected an `{Trait}` closure, found `{Self}`"
)]
#[fundamental]
#[must_use = "closures are lazy and do nothing unless called"]
pub trait FnOnce<Args : Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
