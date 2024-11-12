use crate::bind::DFOpaqueValue;
use crate::prelude::*;
use crate::core::ops::*;


#[derive(Clone)]
#[repr(transparent)]
pub struct Matrix<const ROWS : usize, const COLUMNS : usize> {
    inner : List<Float>
}

impl<const ROWS : usize, const COLUMNS : usize> Matrix<ROWS, COLUMNS> {

    pub fn zero() -> Self { unsafe {
        extern "C" {
            fn DF_ACTION__Repeat_Multiple( count : UInt ) -> ();
        }
        let inner = DF_ACTION__SetVariable_CreateList();
        DF_ACTION__Repeat_Multiple(UInt::from(COLUMNS * ROWS)); DF_BRACKET__Repeat_Open();
            DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), Float::from(0.0).to_opaque());
        DF_BRACKET__Repeat_Close();
        Matrix { inner }
    } }

    #[inline(always)]
    pub fn into_inner(self) -> List<Float> { self.inner }

}

impl<const SIZE : usize> Matrix<SIZE, SIZE> {

    pub fn identity() -> Self { unsafe {
        extern "C" {
            fn DF_ACTION__IfVariable_Specialcharequals( a : UInt, b : UInt ) -> ();
        }
        let inner = DF_ACTION__SetVariable_CreateList();
        let row = DF_ACTION__Repeat_Multiple(UInt::from(SIZE)); DF_BRACKET__Repeat_Open();
            let column = DF_ACTION__Repeat_Multiple(UInt::from(SIZE)); DF_BRACKET__Repeat_Open();
                DF_ACTION__IfVariable_Specialcharequals(column, row); DF_BRACKET__Normal_Open();
                    DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), Float::from(1.0).to_opaque());
                DF_BRACKET__Normal_Close();
                DF_ELSE(); DF_BRACKET__Normal_Open();
                    DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), Float::from(0.0).to_opaque());
                DF_BRACKET__Normal_Close();
            DF_BRACKET__Repeat_Close();
        DF_BRACKET__Repeat_Close();
        Matrix { inner }
    } }

}

impl Matrix<4, 4> {

    #[inline(always)]
    pub fn translation<F0 : Into<Float>, F1 : Into<Float>, F2 : Into<Float>>(x : F0, y : F1, z : F2) -> Self { unsafe {
        Matrix { inner : DF_ACTION__SetVariable_CreateList(
            Float::from(1.0),
                Float::from(0.0),
                    Float::from(0.0),
                        x.into(),
            Float::from(0.0),
                Float::from(1.0),
                    Float::from(0.0),
                        y.into(),
            Float::from(0.0),
                Float::from(0.0),
                    Float::from(1.0),
                        z.into(),
            Float::from(0.0),
                Float::from(0.0),
                    Float::from(0.0),
                        Float::from(1.0)
        ) }
    } }

    #[inline(always)]
    pub fn scale<F0 : Into<Float>, F1 : Into<Float>, F2 : Into<Float>>(x : F0, y : F1, z : F2) -> Self { unsafe {
        Matrix { inner : DF_ACTION__SetVariable_CreateList(
            x.into(),
                Float::from(0.0),
                    Float::from(0.0),
                        Float::from(0.0),
            Float::from(0.0),
                y.into(),
                    Float::from(0.0),
                        Float::from(0.0),
            Float::from(0.0),
                Float::from(0.0),
                    z.into(),
                        Float::from(0.0),
            Float::from(0.0),
                Float::from(0.0),
                    Float::from(0.0),
                        Float::from(1.0)
        ) }
    } }

}

impl<const ROWS : usize, const COLUMNS : usize> Add<Matrix<ROWS, COLUMNS>> for Matrix<ROWS, COLUMNS> {
    type Output = Matrix<ROWS, COLUMNS>;
    fn add(self, rhs : Matrix<ROWS, COLUMNS>) -> Self::Output { unsafe {
        let inner = DF_ACTION__SetVariable_CreateList();
        let row = DF_ACTION__Repeat_Multiple(UInt::from(ROWS)); DF_BRACKET__Repeat_Open();
            let column = DF_ACTION__Repeat_Multiple(UInt::from(COLUMNS)); DF_BRACKET__Repeat_Open();
                let i = column + (row - 1usize) * COLUMNS;
                let a = DF_ACTION__SetVariable_GetListValue(self.inner.to_opaque(), i);
                let b = DF_ACTION__SetVariable_GetListValue(rhs.inner.to_opaque(), i);
                DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), (a + b).to_opaque());
            DF_BRACKET__Repeat_Close();
        DF_BRACKET__Repeat_Close();
        Matrix { inner }
    } }
}

impl<const A_ROWS : usize, const A_COLUMNS_B_ROWS : usize, const B_COLUMNS : usize> Mul<Matrix<A_COLUMNS_B_ROWS, B_COLUMNS>> for Matrix<A_ROWS, A_COLUMNS_B_ROWS> {
    type Output = Matrix<A_ROWS, B_COLUMNS>;
    fn mul(self, rhs : Matrix<A_COLUMNS_B_ROWS, B_COLUMNS>) -> Self::Output { unsafe {
        let inner = DF_ACTION__SetVariable_CreateList();
        let row = DF_ACTION__Repeat_Multiple(UInt::from(A_ROWS)); DF_BRACKET__Repeat_Open();
            let a_i0 = (row - 1usize) * A_COLUMNS_B_ROWS;
            let column = DF_ACTION__Repeat_Multiple(UInt::from(B_COLUMNS)); DF_BRACKET__Repeat_Open();
                let cell = DF_TEMPVAR();
                DF_ACTION__SetVariable_Specialcharequals(cell.to_opaque(), Float::from(0.0));
                let i = DF_ACTION__Repeat_Multiple(UInt::from(A_COLUMNS_B_ROWS)); DF_BRACKET__Repeat_Open();
                    let a = DF_ACTION__SetVariable_GetListValue(self.inner.to_opaque(), a_i0 + i);
                    let b = DF_ACTION__SetVariable_GetListValue(rhs.inner.to_opaque(), column + (i - 1usize) * B_COLUMNS);
                    DF_ACTION__SetVariable_SpecialcharplusSpecialcharequals(cell.to_opaque(), a * b);
                DF_BRACKET__Repeat_Close();
                DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), cell);
            DF_BRACKET__Repeat_Close();
        DF_BRACKET__Repeat_Close();
        Matrix { inner }
    } }
}

impl<const ROWS : usize, const COLUMNS : usize> Matrix<ROWS, COLUMNS> {
    pub fn transpose(self) -> Matrix<COLUMNS, ROWS> { unsafe {
        let inner = DF_ACTION__SetVariable_CreateList();
        let row = DF_ACTION__Repeat_Multiple(UInt::from(COLUMNS)); DF_BRACKET__Repeat_Open();
            let column = DF_ACTION__Repeat_Multiple(UInt::from(ROWS)); DF_BRACKET__Repeat_Open();
                let i = row + (column - 1usize) * COLUMNS;
                let a = DF_ACTION__SetVariable_GetListValue(self.inner.to_opaque(), i);
                DF_ACTION__SetVariable_AppendValue(inner.to_opaque(), a.to_opaque());
            DF_BRACKET__Repeat_Close();
        DF_BRACKET__Repeat_Close();
        Matrix { inner }
    } }
}


extern "C" {

    fn DF_TEMPVAR() -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_CreateList( ... ) -> List<Float>;
    fn DF_ACTION__SetVariable_AppendValue( list : DFOpaqueValue, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_GetListValue( list : DFOpaqueValue, index : UInt ) -> Float;
    fn DF_ACTION__SetVariable_Specialcharequals( var : DFOpaqueValue, value : Float ) -> ();
    fn DF_ACTION__SetVariable_SpecialcharplusSpecialcharequals( var : DFOpaqueValue, b : Float );

    fn DF_ACTION__Repeat_Multiple( count : UInt ) -> UInt;

    fn DF_BRACKET__Normal_Open() -> ();
    fn DF_BRACKET__Normal_Close() -> ();
    fn DF_BRACKET__Repeat_Open() -> ();
    fn DF_BRACKET__Repeat_Close() -> ();
    fn DF_ELSE() -> ();

}
