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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RelationshipDirection',
//     name: "RelationshipDirection",
//     owned_attribute: {
//         "RelationshipDirection-Backward": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RelationshipDirection-Backward',
//                 name: "Backward",
//                 _classifier: "RelationshipDirection",
//                 _enumeration: "RelationshipDirection",
//                 litteral_name: "Backward",
//                 litteral_designation: "Backward",
//             },
//         ),
//         "RelationshipDirection-Both": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RelationshipDirection-Both',
//                 name: "Both",
//                 _classifier: "RelationshipDirection",
//                 _enumeration: "RelationshipDirection",
//                 litteral_name: "Both",
//                 litteral_designation: "Both",
//             },
//         ),
//         "RelationshipDirection-Forward": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RelationshipDirection-Forward',
//                 name: "Forward",
//                 _classifier: "RelationshipDirection",
//                 _enumeration: "RelationshipDirection",
//                 litteral_name: "Forward",
//                 litteral_designation: "Forward",
//             },
//         ),
//         "RelationshipDirection-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-RelationshipDirection-None',
//                 name: "None",
//                 _classifier: "RelationshipDirection",
//                 _enumeration: "RelationshipDirection",
//                 litteral_name: "None",
//                 litteral_designation: "None",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#RelationshipDirection",
//     table_name: "bpmn_20_relationship_direction",
//     model_name: "RelationshipDirection",
//     full_name: "bpmn_20_enumeration_relationship_direction",
// }

