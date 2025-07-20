//! bpmn_20_enumeration_event_based_gateway_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum EventBasedGatewayType {
    /// ENUMERATION LITERAL : BPMN20-EventBasedGatewayType-Exclusive
    #[default]
    #[sea_orm(string_value = "Exclusive")]
    Exclusive,
    /// ENUMERATION LITERAL : BPMN20-EventBasedGatewayType-Parallel
    #[sea_orm(string_value = "Parallel")]
    Parallel,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdReference {
//         object_id: "EventBasedGatewayType",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "EventBasedGatewayType",
//     owned_attribute: {
//         "-EventBasedGatewayType-Exclusive": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "EventBasedGatewayType-Exclusive",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Exclusive",
//                 classifier: "EventBasedGatewayType",
//                 enumeration: "EventBasedGatewayType",
//             },
//         ),
//         "-EventBasedGatewayType-Parallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "EventBasedGatewayType-Parallel",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
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

