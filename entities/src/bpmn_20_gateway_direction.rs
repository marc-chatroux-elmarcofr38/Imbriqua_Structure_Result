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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GatewayDirection',
//     name: "GatewayDirection",
//     owned_attribute: {
//         "GatewayDirection-Converging": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GatewayDirection-Converging',
//                 name: "Converging",
//                 _classifier: "GatewayDirection",
//                 _enumeration: "GatewayDirection",
//                 litteral_name: "Converging",
//                 litteral_designation: "Converging",
//             },
//         ),
//         "GatewayDirection-Diverging": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GatewayDirection-Diverging',
//                 name: "Diverging",
//                 _classifier: "GatewayDirection",
//                 _enumeration: "GatewayDirection",
//                 litteral_name: "Diverging",
//                 litteral_designation: "Diverging",
//             },
//         ),
//         "GatewayDirection-Mixed": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GatewayDirection-Mixed',
//                 name: "Mixed",
//                 _classifier: "GatewayDirection",
//                 _enumeration: "GatewayDirection",
//                 litteral_name: "Mixed",
//                 litteral_designation: "Mixed",
//             },
//         ),
//         "GatewayDirection-Unspecified": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-GatewayDirection-Unspecified',
//                 name: "Unspecified",
//                 _classifier: "GatewayDirection",
//                 _enumeration: "GatewayDirection",
//                 litteral_name: "Unspecified",
//                 litteral_designation: "Unspecified",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#GatewayDirection",
//     table_name: "bpmn_20_gateway_direction",
//     model_name: "GatewayDirection",
//     full_name: "bpmn_20_enumeration_gateway_direction",
// }

