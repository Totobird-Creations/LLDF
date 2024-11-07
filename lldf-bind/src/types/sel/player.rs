use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct PlayerSel {
    pub(crate) uuids : List<String>
}

impl PlayerSel {

    #[doc(hidden)]
    pub unsafe fn from_uuids(uuids : DFOpaqueValue) -> Self { unsafe {
        Self { uuids : DF_TRANSMUTE__ListString(uuids) }
    } }

}

impl PlayerSel {

    #[inline(always)]
    pub fn uuids<'l>(&'l self) -> &'l List<String> { &self.uuids }

    #[inline(always)]
    pub fn names(&self) -> &List<Text> { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        let names = crate::bind::gamevalue::DF_GAMEVALUE__SelectionTargetNames_Default();
        DF_ACTION__SelectObject_Reset();
        transmute_unchecked(&DF_TRANSMUTE__ListText(names))
    } }

}

/// `PLAYER_ACTION` / `Item Management`
impl PlayerSel {

    #[inline(always)]
    pub fn give_item(&self, item : Item) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_GiveItems(item.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

}

impl PlayerSel {

    #[inline(always)]
    pub fn send_message<T : DFValue>(&self, text : T) -> () { unsafe {
        DF_ACTION__SelectObject_PlayerName(self.uuids.clone());
        DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False(text.to_opaque());
        DF_ACTION__SelectObject_Reset();
    } }

}


unsafe impl DFSel for PlayerSel {}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__ListString( from : DFOpaqueValue ) -> List<String>;
    fn DF_TRANSMUTE__ListText( from : DFOpaqueValue ) -> List<Text>;

    fn DF_ACTION__SelectObject_PlayerName( target : List<String> ) -> ();
    fn DF_ACTION__SelectObject_Reset( ) -> ();


    fn DF_ACTION__PlayerAction_GiveItems( item : DFOpaqueValue ) -> ();

    fn DF_ACTION__PlayerAction_SendMessage_AlignmentMode_Regular_TextValueMerging_NoSpaces_InheritStyles_False( message : DFOpaqueValue ) -> ();

}
