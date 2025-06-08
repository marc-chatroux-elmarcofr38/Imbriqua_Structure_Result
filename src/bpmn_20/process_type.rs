//! process_type
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ProcessType (Enumeration : ProcessType)
#[derive(Debug, Clone)]
pub enum process_type {
    /// 'none' from (id : 'ProcessType-None', name : 'None')
    none, 
    /// 'public' from (id : 'ProcessType-Public', name : 'Public')
    public, 
    /// 'private' from (id : 'ProcessType-Private', name : 'Private')
    private, 
}
