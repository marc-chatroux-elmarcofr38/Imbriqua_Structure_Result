//! bpmndi_enumeration_message_visible_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MessageVisibleKind {
    /// ENUMERATION LITERAL : MessageVisibleKind-initiating
    #[default]
    #[sea_orm(string_value = "initiating")]
    Initiating,
    /// ENUMERATION LITERAL : MessageVisibleKind-non_initiating
    #[sea_orm(string_value = "non_initiating")]
    NonInitiating,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "MessageVisibleKind",
//     name: "MessageVisibleKind",
//     owned_attribute: {
//         "MessageVisibleKind-initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MessageVisibleKind-initiating",
//                 name: "initiating",
//                 classifier: "MessageVisibleKind",
//                 enumeration: "MessageVisibleKind",
//             },
//         ),
//         "MessageVisibleKind-non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MessageVisibleKind-non_initiating",
//                 name: "non_initiating",
//                 classifier: "MessageVisibleKind",
//                 enumeration: "MessageVisibleKind",
//             },
//         ),
//     },
//     technical_name: "BPMNDI.cmof#MessageVisibleKind",
//     table_name: "bpmndi_message_visible_kind",
//     model_name: "MessageVisibleKind",
//     full_name: "bpmndi_enumeration_message_visible_kind",
// }

