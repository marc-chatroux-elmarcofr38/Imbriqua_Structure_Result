//! bpmn_20_enumeration_multi_instance_behavior

use sea_orm::entity::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MultiInstanceBehavior {
    #[sea_orm(string_value = "None")]
    None,
    #[sea_orm(string_value = "One")]
    One,
    #[sea_orm(string_value = "All")]
    All,
    #[sea_orm(string_value = "Complex")]
    Complex,
}

// RAW :
// CMOFEnumeration {
//     xmi_id: "MultiInstanceBehavior",
//     name: "MultiInstanceBehavior",
//     owned_attribute: [
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MultiInstanceBehavior-None",
//                 name: "None",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MultiInstanceBehavior-One",
//                 name: "One",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MultiInstanceBehavior-All",
//                 name: "All",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "MultiInstanceBehavior-Complex",
//                 name: "Complex",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//     ],
// }

