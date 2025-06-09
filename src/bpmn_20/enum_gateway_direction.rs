//! GatewayDirection
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GatewayDirection (Enumeration : GatewayDirection)
#[derive(Debug, Clone)]
pub enum GatewayDirection {
    /// 'Unspecified' from (id : 'GatewayDirection-Unspecified', name : 'Unspecified')
    Unspecified, 
    /// 'Converging' from (id : 'GatewayDirection-Converging', name : 'Converging')
    Converging, 
    /// 'Diverging' from (id : 'GatewayDirection-Diverging', name : 'Diverging')
    Diverging, 
    /// 'Mixed' from (id : 'GatewayDirection-Mixed', name : 'Mixed')
    Mixed, 
}
