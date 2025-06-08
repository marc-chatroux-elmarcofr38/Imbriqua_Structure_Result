//! event_based_gateway_type

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of EventBasedGatewayType (Enumeration : EventBasedGatewayType)
#[derive(Debug, Clone)]
pub enum event_based_gateway_type {
    /// 'parallel' from (id : 'EventBasedGatewayType-Parallel', name : 'Parallel')
    parallel, 
    /// 'exclusive' from (id : 'EventBasedGatewayType-Exclusive', name : 'Exclusive')
    exclusive, 
}
