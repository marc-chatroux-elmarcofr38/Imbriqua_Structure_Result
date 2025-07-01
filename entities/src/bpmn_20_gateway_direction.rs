//! bpmn_20_enumeration_gateway_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum GatewayDirection {
    /// ENUMERATION LITERAL : GatewayDirection-Unspecified
    #[default]
    #[sea_orm(string_value = "Unspecified")]
    Unspecified,
    /// ENUMERATION LITERAL : GatewayDirection-Converging
    #[sea_orm(string_value = "Converging")]
    Converging,
    /// ENUMERATION LITERAL : GatewayDirection-Diverging
    #[sea_orm(string_value = "Diverging")]
    Diverging,
    /// ENUMERATION LITERAL : GatewayDirection-Mixed
    #[sea_orm(string_value = "Mixed")]
    Mixed,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "GatewayDirection",
//     name: "GatewayDirection",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "GatewayDirection-Unspecified",
//                 name: "Unspecified",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "GatewayDirection-Converging",
//                 name: "Converging",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "GatewayDirection-Diverging",
//                 name: "Diverging",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "GatewayDirection-Mixed",
//                 name: "Mixed",
//                 classifier: "GatewayDirection",
//                 enumeration: "GatewayDirection",
//             },
//         ),
//     ],
// }

