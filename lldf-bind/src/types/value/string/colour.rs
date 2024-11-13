use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A colour.
#[repr(transparent)]
pub struct Colour {
    hexcode : String
}

impl Clone for Colour {
    #[inline(always)]
    fn clone(&self) -> Self { Self { hexcode : self.hexcode.clone() } }
}


impl Colour {

    /// Creates a `Colour` from a hex code.
    /// 
    /// **No checks are done to make sure this hex code is valid.**
    #[inline(always)]
    pub unsafe fn from_hexcode_unchecked<S : Into<String>>(hexcode : S) -> Self { Self { hexcode : hexcode.into() } }

    #[lldf_bind_proc::dfdoc(SetVariable/Rgbcolor)]
    #[inline(always)]
    pub fn from_rgb<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(r : U0, g : U1, b : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_RGBColor(r.into(), g.into(), b.into()) } } }

    #[lldf_bind_proc::dfdoc(SetVariable/Hsbcolor)]
    #[inline(always)]
    pub fn from_hsb<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(h : U0, s : U1, b : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_HSBColor(h.into(), s.into(), b.into()) } } }

    #[lldf_bind_proc::dfdoc(SetVariable/Hslcolor)]
    #[inline(always)]
    pub fn from_hsl<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(h : U0, s : U1, l : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_HSLColor(h.into(), s.into(), l.into()) } } }

}

impl Colour {

    #[inline(always)]
    pub fn hexcode(&self) -> String { unsafe{ transmute_unchecked(self.hexcode.to_opaque()) } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetColorChannels { ColorChannels = RGB })]
    #[inline(always)]
    pub fn rgb(&self) -> (UInt, UInt, UInt) { unsafe {
        let rgb = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_RGB(self.hexcode.to_opaque()).to_opaque();
        let r = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(rgb, 1usize.into()));
        let g = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(rgb, 2usize.into()));
        let b = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(rgb, 3usize.into()));
        (r, g, b)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetColorChannels { ColorChannels = HSB })]
    #[inline(always)]
    pub fn hsb(&self) -> (Float, Float, Float) { unsafe {
        let hsb = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSB(self.hexcode.to_opaque()).to_opaque();
        let h = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsb, 1usize.into()));
        let s = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsb, 2usize.into()));
        let b = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsb, 3usize.into()));
        (h, s, b)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetColorChannels { ColorChannels = HSL })]
    #[inline(always)]
    pub fn hsl(&self) -> (Float, Float, Float) { unsafe {
        let hsl = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSL(self.hexcode.to_opaque()).to_opaque();
        let h = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsl, 1usize.into()));
        let s = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsl, 2usize.into()));
        let l = transmute_unchecked(DF_ACTION__SetVariable_GetListValue(hsl, 3usize.into()));
        (h, s, l)
    } }

}


//unsafe impl DFValue for Colour {
//    #[inline(always)]
//    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
//        self.hexcode.to_opaque()
//    } }
//}


extern "C" {

    fn DF_ACTION__SetVariable_RGBColor( r : UInt, g : UInt, b : UInt ) -> String;
    fn DF_ACTION__SetVariable_HSBColor( h : UInt, s : UInt, b : UInt ) -> String;
    fn DF_ACTION__SetVariable_HSLColor( h : UInt, s : UInt, l : UInt ) -> String;

    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_RGB( hexcode : DFOpaqueValue ) -> List<UInt>;
    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSB( hexcode : DFOpaqueValue ) -> List<Float>;
    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSL( hexcode : DFOpaqueValue ) -> List<Float>;

    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> DFOpaqueValue;

}

