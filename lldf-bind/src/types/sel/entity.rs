use crate::prelude::*;


#[repr(transparent)]
pub struct EntitySel {
    pub(in super::super) uuids : List<String>
}

#[allow(deprecated)]
impl EntitySel {}

unsafe impl DFSel for EntitySel {}
