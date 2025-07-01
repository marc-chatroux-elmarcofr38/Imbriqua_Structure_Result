//! bpmn_20_enumeration_item_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ItemKind {
    /// ENUMERATION LITERAL : ItemKind-Physical
    #[sea_orm(string_value = "Physical")]
    Physical,
    /// ENUMERATION LITERAL : ItemKind-Information
    #[default]
    #[sea_orm(string_value = "Information")]
    Information,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "ItemKind",
//     name: "ItemKind",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ItemKind-Physical",
//                 name: "Physical",
//                 classifier: "ItemKind",
//                 enumeration: "ItemKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ItemKind-Information",
//                 name: "Information",
//                 classifier: "ItemKind",
//                 enumeration: "ItemKind",
//             },
//         ),
//     ],
// }

