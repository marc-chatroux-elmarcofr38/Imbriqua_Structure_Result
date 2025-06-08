//! gateway_direction
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GatewayDirection (Enumeration : GatewayDirection)
#[derive(Debug, Clone)]
pub enum gateway_direction {
    /// 'unspecified' from (id : 'GatewayDirection-Unspecified', name : 'Unspecified')
    unspecified, 
    /// 'converging' from (id : 'GatewayDirection-Converging', name : 'Converging')
    converging, 
    /// 'diverging' from (id : 'GatewayDirection-Diverging', name : 'Diverging')
    diverging, 
    /// 'mixed' from (id : 'GatewayDirection-Mixed', name : 'Mixed')
    mixed, 
}
