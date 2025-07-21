//! bpmn_20_enumeration_relationship_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum RelationshipDirection {
    /// ENUMERATION LITERAL : BPMN20-RelationshipDirection-Backward
    #[sea_orm(string_value = "Backward")]
    Backward,
    /// ENUMERATION LITERAL : BPMN20-RelationshipDirection-Both
    #[sea_orm(string_value = "Both")]
    Both,
    /// ENUMERATION LITERAL : BPMN20-RelationshipDirection-Forward
    #[sea_orm(string_value = "Forward")]
    Forward,
    /// ENUMERATION LITERAL : BPMN20-RelationshipDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdLocalReference {
//         object_id: "RelationshipDirection",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "RelationshipDirection",
//     owned_attribute: {
//         "RelationshipDirection-Backward": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "RelationshipDirection-Backward",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Backward",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         "RelationshipDirection-Both": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "RelationshipDirection-Both",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Both",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         "RelationshipDirection-Forward": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "RelationshipDirection-Forward",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Forward",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//         "RelationshipDirection-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "RelationshipDirection-None",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "None",
//                 classifier: "RelationshipDirection",
//                 enumeration: "RelationshipDirection",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#RelationshipDirection",
//     table_name: "bpmn_20_relationship_direction",
//     model_name: "RelationshipDirection",
//     full_name: "bpmn_20_enumeration_relationship_direction",
// }

