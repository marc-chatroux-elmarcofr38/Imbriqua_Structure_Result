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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ItemKind',
//     name: "ItemKind",
//     owned_attribute: {
//         "ItemKind-Information": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ItemKind-Information',
//                 name: "Information",
//                 _classifier: "ItemKind",
//                 _enumeration: "ItemKind",
//                 litteral_name: "Information",
//                 litteral_designation: "Information",
//             },
//         ),
//         "ItemKind-Physical": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ItemKind-Physical',
//                 name: "Physical",
//                 _classifier: "ItemKind",
//                 _enumeration: "ItemKind",
//                 litteral_name: "Physical",
//                 litteral_designation: "Physical",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ItemKind",
//     table_name: "bpmn_20_item_kind",
//     model_name: "ItemKind",
//     full_name: "bpmn_20_enumeration_item_kind",
// }

