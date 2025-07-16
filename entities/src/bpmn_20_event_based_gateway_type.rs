//! bpmn_20_enumeration_event_based_gateway_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum EventBasedGatewayType {
    /// ENUMERATION LITERAL : EventBasedGatewayType-Exclusive
    #[default]
    #[sea_orm(string_value = "Exclusive")]
    Exclusive,
    /// ENUMERATION LITERAL : EventBasedGatewayType-Parallel
    #[sea_orm(string_value = "Parallel")]
    Parallel,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "EventBasedGatewayType",
//     name: "EventBasedGatewayType",
//     owned_attribute: {
//         "EventBasedGatewayType-Exclusive": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "EventBasedGatewayType-Exclusive",
//                 name: "Exclusive",
//                 classifier: "EventBasedGatewayType",
//                 enumeration: "EventBasedGatewayType",
//             },
//         ),
//         "EventBasedGatewayType-Parallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "EventBasedGatewayType-Parallel",
//                 name: "Parallel",
//                 classifier: "EventBasedGatewayType",
//                 enumeration: "EventBasedGatewayType",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#EventBasedGatewayType",
//     table_name: "bpmn_20_event_based_gateway_type",
//     model_name: "EventBasedGatewayType",
//     full_name: "bpmn_20_enumeration_event_based_gateway_type",
// }

