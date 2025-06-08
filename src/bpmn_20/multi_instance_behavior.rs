//! multi_instance_behavior

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of MultiInstanceBehavior (Enumeration : MultiInstanceBehavior)
#[derive(Debug, Clone)]
pub enum multi_instance_behavior {
    /// 'none' from (id : 'MultiInstanceBehavior-None', name : 'None')
    none, 
    /// 'one' from (id : 'MultiInstanceBehavior-One', name : 'One')
    one, 
    /// 'all' from (id : 'MultiInstanceBehavior-All', name : 'All')
    all, 
    /// 'complex' from (id : 'MultiInstanceBehavior-Complex', name : 'Complex')
    complex, 
}
