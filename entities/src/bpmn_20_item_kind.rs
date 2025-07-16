//! bpmn_20_enumeration_item_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ItemKind {
    /// ENUMERATION LITERAL : ItemKind-Information
    #[default]
    #[sea_orm(string_value = "Information")]
    Information,
    /// ENUMERATION LITERAL : ItemKind-Physical
    #[sea_orm(string_value = "Physical")]
    Physical,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "ItemKind",
//     name: "ItemKind",
//     owned_attribute: {
//         "ItemKind-Information": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ItemKind-Information",
//                 name: "Information",
//                 classifier: "ItemKind",
//                 enumeration: "ItemKind",
//             },
//         ),
//         "ItemKind-Physical": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ItemKind-Physical",
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

