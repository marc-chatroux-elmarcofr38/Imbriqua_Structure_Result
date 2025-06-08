//! item_kind

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ItemKind (Enumeration : ItemKind)
#[derive(Debug, Clone)]
pub enum item_kind {
    /// 'physical' from (id : 'ItemKind-Physical', name : 'Physical')
    physical, 
    /// 'information' from (id : 'ItemKind-Information', name : 'Information')
    information, 
}
