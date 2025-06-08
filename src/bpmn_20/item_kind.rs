//! item_kind
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ItemKind (Enumeration : ItemKind)
#[derive(Debug, Clone)]
pub enum ItemKind {
    /// 'Physical' from (id : 'ItemKind-Physical', name : 'Physical')
    Physical, 
    /// 'Information' from (id : 'ItemKind-Information', name : 'Information')
    Information, 
}
