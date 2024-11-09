use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
#[repr(transparent)]
pub struct Colour {
    hexcode : String
}


impl Colour {

    #[inline(always)]
    pub unsafe fn from_hexcode_unchecked<S : Into<String>>(hexcode : S) -> Self { Self { hexcode : hexcode.into() } }

    #[lldf_bind_proc::dfdoc(SetVariable/Rgbcolor)]
    #[inline(always)]
    pub fn from_rgb<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(r : U0, g : U1, b : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_RGBColor(r.into(), g.into(), b.into()) } } }

    #[lldf_bind_proc::dfdoc(SetVariable/Hsbcolor)]
    #[inline(always)]
    pub fn from_hsb<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(h : U0, s : U1, b : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_RGBColor(h.into(), s.into(), b.into()) } } }

    #[lldf_bind_proc::dfdoc(SetVariable/Hslcolor)]
    #[inline(always)]
    pub fn from_hsl<U0 : Into<UInt>, U1 : Into<UInt>, U2 : Into<UInt>>(h : U0, s : U1, l : U2) -> Self { Self { hexcode : unsafe{ DF_ACTION__SetVariable_RGBColor(h.into(), s.into(), l.into()) } } }

}

impl Colour {

    #[inline(always)]
    pub fn hexcode(&self) -> String {
        self.hexcode.clone()
    }

    #[inline(always)]
    pub fn rgb(&self) -> (UInt, UInt, UInt) { unsafe {
        let rgb = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_RGB(self.hexcode.clone()).to_opaque();
        let r = DF_TRANSMUTE__UInt(DF_ACTION__SetVariable_GetListValue(rgb.clone(), 1usize.into()));
        let g = DF_TRANSMUTE__UInt(DF_ACTION__SetVariable_GetListValue(rgb.clone(), 2usize.into()));
        let b = DF_TRANSMUTE__UInt(DF_ACTION__SetVariable_GetListValue(rgb, 3usize.into()));
        (r, g, b)
    } }

    #[inline(always)]
    pub fn hsb(&self) -> (Float, Float, Float) { unsafe {
        let hsb = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSB(self.hexcode.clone()).to_opaque();
        let h = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsb.clone(), 1usize.into()));
        let s = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsb.clone(), 2usize.into()));
        let b = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsb, 3usize.into()));
        (h, s, b)
    } }

    #[inline(always)]
    pub fn hsl(&self) -> (Float, Float, Float) { unsafe {
        let hsl = DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSL(self.hexcode.clone()).to_opaque();
        let h = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsl.clone(), 1usize.into()));
        let s = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsl.clone(), 2usize.into()));
        let l = DF_TRANSMUTE__Float(DF_ACTION__SetVariable_GetListValue(hsl, 3usize.into()));
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

    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_RGB( hexcode : String ) -> List<UInt>;
    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSB( hexcode : String ) -> List<Float>;
    fn DF_ACTION__SetVariable_GetColorChannels_ColorChannels_HSL( hexcode : String ) -> List<Float>;

    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> DFOpaqueValue;

    fn DF_TRANSMUTE__UInt( from : DFOpaqueValue ) -> UInt;
    fn DF_TRANSMUTE__Float( from : DFOpaqueValue ) -> Float;

}

