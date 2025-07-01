//! bpmn_20_enumeration_relationship_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum RelationshipDirection {
    /// ENUMERATION LITERAL : RelationshipDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : RelationshipDirection-Forward
    #[sea_orm(string_value = "Forward")]
    Forward,
    /// ENUMERATION LITERAL : RelationshipDirection-Backward
    #[sea_orm(string_value = "Backward")]
    Backward,
    /// ENUMERATION LITERAL : RelationshipDirection-Both
    #[sea_orm(string_value = "Both")]
    Both,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "RelationshipDirection",
//     name: "RelationshipDirection",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "RelationshipDirection-None",
//                 name: "None",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "RelationshipDirection-Forward",
//                 name: "Forward",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "RelationshipDirection-Backward",
//                 name: "Backward",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "RelationshipDirection-Both",
//                 name: "Both",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//     ],
// }

