//! AdHocOrdering
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of AdHocOrdering (Enumeration : AdHocOrdering)
#[derive(Debug, Clone)]
pub enum AdHocOrdering {
    /// 'Parallel' from (id : 'AdHocOrdering-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Sequential' from (id : 'AdHocOrdering-Sequential', name : 'Sequential')
    Sequential, 
}
