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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ProcessType',
//     name: "ProcessType",
//     owned_attribute: {
//         "ProcessType-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ProcessType-None',
//                 name: "None",
//                 _classifier: "ProcessType",
//                 _enumeration: "ProcessType",
//                 litteral_name: "None",
//                 litteral_designation: "None",
//             },
//         ),
//         "ProcessType-Private": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ProcessType-Private',
//                 name: "Private",
//                 _classifier: "ProcessType",
//                 _enumeration: "ProcessType",
//                 litteral_name: "Private",
//                 litteral_designation: "Private",
//             },
//         ),
//         "ProcessType-Public": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-ProcessType-Public',
//                 name: "Public",
//                 _classifier: "ProcessType",
//                 _enumeration: "ProcessType",
//                 litteral_name: "Public",
//                 litteral_designation: "Public",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#ProcessType",
//     table_name: "bpmn_20_process_type",
//     model_name: "ProcessType",
//     full_name: "bpmn_20_enumeration_process_type",
// }

