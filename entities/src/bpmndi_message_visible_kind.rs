//! bpmndi_enumeration_message_visible_kind

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MessageVisibleKind {
    /// ENUMERATION LITERAL : BPMNDI-MessageVisibleKind-initiating
    #[default]
    #[sea_orm(string_value = "initiating")]
    Initiating,
    /// ENUMERATION LITERAL : BPMNDI-MessageVisibleKind-non_initiating
    #[sea_orm(string_value = "non_initiating")]
    NonInitiating,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "Weak ref of "BPMNDI-MessageVisibleKind" (loaded : false)",
//     name: "MessageVisibleKind",
//     owned_attribute: {
//         "-MessageVisibleKind-initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-MessageVisibleKind-initiating" (loaded : false)",
//                 name: "initiating",
//                 classifier: "MessageVisibleKind",
//                 enumeration: "MessageVisibleKind",
//             },
//         ),
//         "-MessageVisibleKind-non_initiating": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMNDI-MessageVisibleKind-non_initiating" (loaded : false)",
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

