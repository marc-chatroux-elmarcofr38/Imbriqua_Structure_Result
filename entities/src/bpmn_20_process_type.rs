//! bpmn_20_enumeration_process_type

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ProcessType {
    /// ENUMERATION LITERAL : ProcessType-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : ProcessType-Public
    #[sea_orm(string_value = "Public")]
    Public,
    /// ENUMERATION LITERAL : ProcessType-Private
    #[sea_orm(string_value = "Private")]
    Private,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "ProcessType",
//     name: "ProcessType",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ProcessType-None",
//                 name: "None",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ProcessType-Public",
//                 name: "Public",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "ProcessType-Private",
//                 name: "Private",
//                 classifier: "ProcessType",
//                 enumeration: "ProcessType",
//             },
//         ),
//     ],
// }

