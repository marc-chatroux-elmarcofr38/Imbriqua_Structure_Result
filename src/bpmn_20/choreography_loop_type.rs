//! choreography_loop_type

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ChoreographyLoopType (Enumeration : ChoreographyLoopType)
#[derive(Debug, Clone)]
pub enum choreography_loop_type {
    /// 'none' from (id : 'ChoreographyLoopType-None', name : 'None')
    none, 
    /// 'standard' from (id : 'ChoreographyLoopType-Standard', name : 'Standard')
    standard, 
    /// 'multi_instance_sequential' from (id : 'ChoreographyLoopType-MultiInstanceSequential', name : 'MultiInstanceSequential')
    multi_instance_sequential, 
    /// 'multi_instance_parallel' from (id : 'ChoreographyLoopType-MultiInstanceParallel', name : 'MultiInstanceParallel')
    multi_instance_parallel, 
}
