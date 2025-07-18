//! bpmn_20_enumeration_association_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssociationDirection {
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-Both
    #[sea_orm(string_value = "Both")]
    Both,
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-One
    #[sea_orm(string_value = "One")]
    One,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdReference {
//         local_id: "AssociationDirection",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "AssociationDirection",
//     owned_attribute: {
//         "-AssociationDirection-Both": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     local_id: "AssociationDirection-Both",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Both",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         "-AssociationDirection-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     local_id: "AssociationDirection-None",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "None",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         "-AssociationDirection-One": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     local_id: "AssociationDirection-One",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "One",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#AssociationDirection",
//     table_name: "bpmn_20_association_direction",
//     model_name: "AssociationDirection",
//     full_name: "bpmn_20_enumeration_association_direction",
// }

