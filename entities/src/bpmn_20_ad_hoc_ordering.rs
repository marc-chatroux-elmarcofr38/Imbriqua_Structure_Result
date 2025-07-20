//! bpmn_20_enumeration_ad_hoc_ordering

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AdHocOrdering {
    /// ENUMERATION LITERAL : BPMN20-AdHocOrdering-Parallel
    #[default]
    #[sea_orm(string_value = "Parallel")]
    Parallel,
    /// ENUMERATION LITERAL : BPMN20-AdHocOrdering-Sequential
    #[sea_orm(string_value = "Sequential")]
    Sequential,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "Weak ref of "BPMN20-AdHocOrdering" (loaded : false)",
//     name: "AdHocOrdering",
//     owned_attribute: {
//         "-AdHocOrdering-Parallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-AdHocOrdering-Parallel" (loaded : false)",
//                 name: "Parallel",
//                 classifier: "AdHocOrdering",
//                 enumeration: "AdHocOrdering",
//             },
//         ),
//         "-AdHocOrdering-Sequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Weak ref of "BPMN20-AdHocOrdering-Sequential" (loaded : false)",
//                 name: "Sequential",
//                 classifier: "AdHocOrdering",
//                 enumeration: "AdHocOrdering",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#AdHocOrdering",
//     table_name: "bpmn_20_ad_hoc_ordering",
//     model_name: "AdHocOrdering",
//     full_name: "bpmn_20_enumeration_ad_hoc_ordering",
// }

