//! bpmn_20_enumeration_ad_hoc_ordering

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AdHocOrdering {
    #[sea_orm(string_value = "Parallel")]
    Parallel,
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

