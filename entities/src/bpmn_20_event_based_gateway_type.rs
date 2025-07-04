//! bpmn_20_enumeration_event_based_gateway_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum EventBasedGatewayType {
    /// ENUMERATION LITERAL : EventBasedGatewayType-Parallel
    #[sea_orm(string_value = "Parallel")]
    Parallel,
    /// ENUMERATION LITERAL : EventBasedGatewayType-Exclusive
    #[default]
    #[sea_orm(string_value = "Exclusive")]
    Exclusive,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "EventBasedGatewayType",
//     name: "EventBasedGatewayType",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "EventBasedGatewayType-Parallel",
//                 name: "Parallel",
//                 classifier: "EventBasedGatewayType",
//                 enumeration: "EventBasedGatewayType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "EventBasedGatewayType-Exclusive",
//                 name: "Exclusive",
//                 classifier: "EventBasedGatewayType",
//                 enumeration: "EventBasedGatewayType",
//             },
//         ),
//     ],
// }

