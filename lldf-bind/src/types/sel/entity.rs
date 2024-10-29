use crate::prelude::*;


#[repr(transparent)]
pub struct EntitySel {
    pub(in super::super) uuids : *const List<String>
}

#[allow(deprecated)]
impl EntitySel {}

unsafe impl DFSel for EntitySel {}
