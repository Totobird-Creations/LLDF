use super::*;


#[repr(transparent)]
pub struct Float {
    pub(crate) _opaque_type : f64
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32> for Float { #[inline(always)] fn from(value : f32) -> Self { Self { _opaque_type : value as f64 } } } // TODO: test this
impl From<f64> for Float { #[inline(always)] fn from(value : f64) -> Self { Self { _opaque_type : value } } }
impl From<UInt> for Float { #[inline(always)] fn from(value : UInt) -> Self { Self { _opaque_type : value._opaque_type as f64 } } }
impl From<Int> for Float { #[inline(always)] fn from(value : Int) -> Self { Self { _opaque_type : value._opaque_type as f64 } } }

impl Float {

    pub const E               : Float = Float { _opaque_type : 2.71828182845904523536028747135266250_f64 };
    pub const FRAC_1_PI       : Float = Float { _opaque_type : 0.318309886183790671537767526745028724_f64 };
    pub const FRAC_1_SQRT_2   : Float = Float { _opaque_type : 0.707106781186547524400844362104849039_f64 };
    pub const FRAC_2_PI       : Float = Float { _opaque_type : 0.636619772367581343075535053490057448_f64 };
    pub const FRAC_2_SQRT_PI  : Float = Float { _opaque_type : 1.12837916709551257389615890312154517_f64 };
    pub const FRAC_PI_2       : Float = Float { _opaque_type : 1.57079632679489661923132169163975144_f64 };
    pub const FRAC_PI_3       : Float = Float { _opaque_type : 1.04719755119659774615421446109316763_f64 };
    pub const FRAC_PI_4       : Float = Float { _opaque_type : 0.785398163397448309615660845819875721_f64 };
    pub const FRAC_PI_6       : Float = Float { _opaque_type : 0.52359877559829887307710723054658381_f64 };
    pub const FRAC_PI_8       : Float = Float { _opaque_type : 0.39269908169872415480783042290993786_f64 };
    pub const LN_2            : Float = Float { _opaque_type : 0.693147180559945309417232121458176568_f64 };
    pub const LN_10           : Float = Float { _opaque_type : 2.30258509299404568401799145468436421_f64 };
    pub const LOG2_10         : Float = Float { _opaque_type : 3.32192809488736234787031942948939018_f64 };
    pub const LOG10_2         : Float = Float { _opaque_type : 0.301029995663981195213738894724493027_f64 };
    pub const LOG10_E         : Float = Float { _opaque_type : 0.434294481903251827651128918916605082_f64 };
    pub const PI              : Float = Float { _opaque_type : 3.14159265358979323846264338327950288_f64 };
    pub const SQRT_2          : Float = Float { _opaque_type : 1.41421356237309504880168872420969808_f64 };
    pub const TAU             : Float = Float { _opaque_type : 6.28318530717958647692528676655900577_f64 };
    pub const EGAMMA          : Float = Float { _opaque_type : 0.577215664901532860606512090082402431_f64 };
    pub const FRAC_1_SQRT_3   : Float = Float { _opaque_type : 0.577350269189625764509148780501957456_f64 };
    pub const FRAC_1_SQRT_2PI : Float = Float { _opaque_type : 0.398942280401432677939946059934381868_f64 };
    pub const FRAC_1_SQRT_PI  : Float = Float { _opaque_type : 0.564189583547756286948079451560772586_f64 };
    pub const PHI             : Float = Float { _opaque_type : 1.618033988749894848204586834365638118_f64 };
    pub const SQRT_3          : Float = Float { _opaque_type : 1.732050807568877293527446341505872367_f64 };

    pub const FRAC_PI_180 : Float = Float { _opaque_type : 0.017453292519943295474371680597869272_f64 };
    pub const FRAC_180_PI : Float = Float { _opaque_type : 57.295779513082322864647721871733665466_f64 };

}

impl<T : Into<Float>> Add<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_Specialcharplus(
            self.to_opaque(),
            rhs.into().to_opaque()
        )
    } }
}

// TODO: Sub

impl<T : Into<Float>> Mul<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_x(
            self.to_opaque(),
            rhs.into().to_opaque()
        )
    } }
}

// TODO: Div

// TODO: Rem

impl Float {

    // TODO: pow

    // TODO: sqrt

    // TODO: cbrt

    // TODO: root

    // TODO: ln

    // TODO: log2

    // TODO: log10

    // TODO: log

    // TODO: abs

    // TODO: clamp

    // TODO: wrap

    // TODO: bounce
    
    // TODO: midpoint

    // TODO: random
    
    // TODO: average

    // TODO: floor

    // TODO: ceil

    // TODO: nearest

    // TODO: trunc

    // TODO: min

    // TODO: max

    // TODO: normaldist

    // TODO: normaldist_fold

    // TODO: sin

    // TODO: arcsin

    // TODO: sinh

    // TODO: cos

    // TODO: arccos

    // TODO: cosh

    // TODO: tan

    // TODO: arctan

    // TODO: tanh

    #[inline(always)]
    pub fn deg_to_rad(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_x(self.to_opaque(), Self::FRAC_PI_180.to_opaque())
    } }

    #[inline(always)]
    pub fn rad_to_deg(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_x(self.to_opaque(), Self::FRAC_180_PI.to_opaque())
    } }

}

unsafe impl DFValue for Float {
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type)
    } }
}
impl Into<Float> for Float {
    #[inline(always)]
    fn into(self) -> Float { self }
}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( left : DFOpaqueValue, right : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_x( left : DFOpaqueValue, right : DFOpaqueValue ) -> Float;

}
