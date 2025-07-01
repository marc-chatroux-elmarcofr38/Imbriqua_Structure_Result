//! bpmn_20_enumeration_ad_hoc_ordering

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AdHocOrdering {
    /// ENUMERATION LITERAL : AdHocOrdering-Parallel
    #[default]
    #[sea_orm(string_value = "Parallel")]
    Parallel,
    /// ENUMERATION LITERAL : AdHocOrdering-Sequential
    #[sea_orm(string_value = "Sequential")]
    Sequential,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "AdHocOrdering",
//     name: "AdHocOrdering",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AdHocOrdering-Parallel",
//                 name: "Parallel",
//                 classifier: "AdHocOrdering",
//                 enumeration: "AdHocOrdering",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AdHocOrdering-Sequential",
//                 name: "Sequential",
//                 classifier: "AdHocOrdering",
//                 enumeration: "AdHocOrdering",
//             },
//         ),
//     ],
// }

