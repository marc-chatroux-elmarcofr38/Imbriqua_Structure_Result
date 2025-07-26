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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AdHocOrdering',
//     name: "AdHocOrdering",
//     owned_attribute: {
//         "AdHocOrdering-Parallel": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AdHocOrdering-Parallel',
//                 name: "Parallel",
//                 _classifier: "AdHocOrdering",
//                 _enumeration: "AdHocOrdering",
//                 litteral_name: "Parallel",
//                 litteral_designation: "Parallel",
//             },
//         ),
//         "AdHocOrdering-Sequential": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AdHocOrdering-Sequential',
//                 name: "Sequential",
//                 _classifier: "AdHocOrdering",
//                 _enumeration: "AdHocOrdering",
//                 litteral_name: "Sequential",
//                 litteral_designation: "Sequential",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#AdHocOrdering",
//     table_name: "bpmn_20_ad_hoc_ordering",
//     model_name: "AdHocOrdering",
//     full_name: "bpmn_20_enumeration_ad_hoc_ordering",
// }

