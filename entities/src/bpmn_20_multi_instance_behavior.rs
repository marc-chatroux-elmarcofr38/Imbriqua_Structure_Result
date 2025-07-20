//! bpmn_20_enumeration_multi_instance_behavior

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MultiInstanceBehavior {
    /// ENUMERATION LITERAL : BPMN20-MultiInstanceBehavior-All
    #[default]
    #[sea_orm(string_value = "All")]
    All,
    /// ENUMERATION LITERAL : BPMN20-MultiInstanceBehavior-Complex
    #[sea_orm(string_value = "Complex")]
    Complex,
    /// ENUMERATION LITERAL : BPMN20-MultiInstanceBehavior-None
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : BPMN20-MultiInstanceBehavior-One
    #[sea_orm(string_value = "One")]
    One,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: XMIIdReference {
//         object_id: "MultiInstanceBehavior",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "MultiInstanceBehavior",
//     owned_attribute: {
//         "-MultiInstanceBehavior-All": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "MultiInstanceBehavior-All",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "All",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         "-MultiInstanceBehavior-Complex": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "MultiInstanceBehavior-Complex",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "Complex",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         "-MultiInstanceBehavior-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "MultiInstanceBehavior-None",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "None",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//         "-MultiInstanceBehavior-One": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: XMIIdReference {
//                     object_id: "MultiInstanceBehavior-One",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "One",
//                 classifier: "MultiInstanceBehavior",
//                 enumeration: "MultiInstanceBehavior",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#MultiInstanceBehavior",
//     table_name: "bpmn_20_multi_instance_behavior",
//     model_name: "MultiInstanceBehavior",
//     full_name: "bpmn_20_enumeration_multi_instance_behavior",
// }

