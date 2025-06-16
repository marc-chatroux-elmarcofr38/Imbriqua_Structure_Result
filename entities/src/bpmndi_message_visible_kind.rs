//! bpmndi_enumeration_message_visible_kind

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MessageVisibleKind {
    #[sea_orm(string_value = "initiating")]
    Initiating,
    #[sea_orm(string_value = "non_initiating")]
    NonInitiating,
}

// RAW :
// CMOFEnumeration {
//     xmi_id: "MessageVisibleKind",
//     name: "MessageVisibleKind",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MessageVisibleKind-initiating",
//                 name: "initiating",
//                 classifier: "MessageVisibleKind",
//                 enumeration: "MessageVisibleKind",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MessageVisibleKind-non_initiating",
//                 name: "non_initiating",
//                 classifier: "MessageVisibleKind",
//                 enumeration: "MessageVisibleKind",
//             },
//         ),
//     ],
// }

