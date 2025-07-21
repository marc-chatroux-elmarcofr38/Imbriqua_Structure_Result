//! bpmn_20_enumeration_item_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ItemKind {
    /// ENUMERATION LITERAL : BPMN20-ItemKind-Information
    #[default]
    #[sea_orm(string_value = "Information")]
    Information,
    /// ENUMERATION LITERAL : BPMN20-ItemKind-Physical
    #[sea_orm(string_value = "Physical")]
    Physical,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdLocalReference {
//         object_id: "ItemKind",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "ItemKind",
//     owned_attribute: {
//         "ItemKind-Information": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ItemKind-Information",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Information",
//                 classifier: "ItemKind",
//                 enumeration: "ItemKind",
//             },
//         ),
//         "ItemKind-Physical": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "ItemKind-Physical",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Physical",
//                 classifier: "ItemKind",
//                 enumeration: "ItemKind",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ItemKind",
//     table_name: "bpmn_20_item_kind",
//     model_name: "ItemKind",
//     full_name: "bpmn_20_enumeration_item_kind",
// }

