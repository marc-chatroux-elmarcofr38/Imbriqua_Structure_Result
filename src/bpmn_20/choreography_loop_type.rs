//! choreography_loop_type
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ChoreographyLoopType (Enumeration : ChoreographyLoopType)
#[derive(Debug, Clone)]
pub enum ChoreographyLoopType {
    /// 'None' from (id : 'ChoreographyLoopType-None', name : 'None')
    None, 
    /// 'Standard' from (id : 'ChoreographyLoopType-Standard', name : 'Standard')
    Standard, 
    /// 'MultiInstanceSequential' from (id : 'ChoreographyLoopType-MultiInstanceSequential', name : 'MultiInstanceSequential')
    MultiInstanceSequential, 
    /// 'MultiInstanceParallel' from (id : 'ChoreographyLoopType-MultiInstanceParallel', name : 'MultiInstanceParallel')
    MultiInstanceParallel, 
}
