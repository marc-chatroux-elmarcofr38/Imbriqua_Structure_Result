//! bpmn_20_enumeration_gateway_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum GatewayDirection {
    /// ENUMERATION LITERAL : BPMN20-GatewayDirection-Converging
    #[sea_orm(string_value = "Converging")]
    Converging,
    /// ENUMERATION LITERAL : BPMN20-GatewayDirection-Diverging
    #[sea_orm(string_value = "Diverging")]
    Diverging,
    /// ENUMERATION LITERAL : BPMN20-GatewayDirection-Mixed
    #[sea_orm(string_value = "Mixed")]
    Mixed,
    /// ENUMERATION LITERAL : BPMN20-GatewayDirection-Unspecified
    #[default]
    #[sea_orm(string_value = "Unspecified")]
    Unspecified,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdReference {
//         object_id: "GatewayDirection",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "GatewayDirection",
//     owned_attribute: {
//         "-GatewayDirection-Converging": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "GatewayDirection-Converging",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Converging",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         "-GatewayDirection-Diverging": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "GatewayDirection-Diverging",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Diverging",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         "-GatewayDirection-Mixed": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "GatewayDirection-Mixed",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Mixed",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         "-GatewayDirection-Unspecified": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "GatewayDirection-Unspecified",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Unspecified",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#GatewayDirection",
//     table_name: "bpmn_20_gateway_direction",
//     model_name: "GatewayDirection",
//     full_name: "bpmn_20_enumeration_gateway_direction",
// }

