//! process_type
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ProcessType (Enumeration : ProcessType)
#[derive(Debug, Clone)]
pub enum ProcessType {
    /// 'None' from (id : 'ProcessType-None', name : 'None')
    None, 
    /// 'Public' from (id : 'ProcessType-Public', name : 'Public')
    Public, 
    /// 'Private' from (id : 'ProcessType-Private', name : 'Private')
    Private, 
}
