//! event_based_gateway_type
#![allow(unused_imports)]

use crate::package_bpmn_20::*;
use crate::Builder;

/// Conversion of EventBasedGatewayType (Enumeration : EventBasedGatewayType)
#[derive(Debug, Clone)]
pub enum EventBasedGatewayType {
    /// 'Parallel' from (id : 'EventBasedGatewayType-Parallel', name : 'Parallel')
    Parallel, 
    /// 'Exclusive' from (id : 'EventBasedGatewayType-Exclusive', name : 'Exclusive')
    Exclusive, 
}
