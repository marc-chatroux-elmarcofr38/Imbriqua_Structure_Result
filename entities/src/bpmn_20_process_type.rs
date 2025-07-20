//! bpmn_20_enumeration_process_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ProcessType {
    /// ENUMERATION LITERAL : BPMN20-ProcessType-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : BPMN20-ProcessType-Private
    #[sea_orm(string_value = "Private")]
    Private,
    /// ENUMERATION LITERAL : BPMN20-ProcessType-Public
    #[sea_orm(string_value = "Public")]
    Public,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "Weak ref of "BPMN20-ProcessType" (loaded : false)",
//     name: "ProcessType",
//     owned_attribute: {
//         "-ProcessType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ProcessType-None" (loaded : false)",
//                 name: "None",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//         "-ProcessType-Private": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ProcessType-Private" (loaded : false)",
//                 name: "Private",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//         "-ProcessType-Public": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-ProcessType-Public" (loaded : false)",
//                 name: "Public",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ProcessType",
//     table_name: "bpmn_20_process_type",
//     model_name: "ProcessType",
//     full_name: "bpmn_20_enumeration_process_type",
// }

