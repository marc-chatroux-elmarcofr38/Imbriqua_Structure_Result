//! ad_hoc_ordering
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of AdHocOrdering (Enumeration : AdHocOrdering)
#[derive(Debug, Clone)]
pub enum ad_hoc_ordering {
    /// 'parallel' from (id : 'AdHocOrdering-Parallel', name : 'Parallel')
    parallel, 
    /// 'sequential' from (id : 'AdHocOrdering-Sequential', name : 'Sequential')
    sequential, 
}
