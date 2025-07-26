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
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-MultiInstanceBehavior',
//     name: "MultiInstanceBehavior",
//     owned_attribute: {
//         "MultiInstanceBehavior-All": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-MultiInstanceBehavior-All',
//                 name: "All",
//                 _classifier: "MultiInstanceBehavior",
//                 _enumeration: "MultiInstanceBehavior",
//                 litteral_name: "All",
//                 litteral_designation: "All",
//             },
//         ),
//         "MultiInstanceBehavior-Complex": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-MultiInstanceBehavior-Complex',
//                 name: "Complex",
//                 _classifier: "MultiInstanceBehavior",
//                 _enumeration: "MultiInstanceBehavior",
//                 litteral_name: "Complex",
//                 litteral_designation: "Complex",
//             },
//         ),
//         "MultiInstanceBehavior-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-MultiInstanceBehavior-None',
//                 name: "None",
//                 _classifier: "MultiInstanceBehavior",
//                 _enumeration: "MultiInstanceBehavior",
//                 litteral_name: "None",
//                 litteral_designation: "None",
//             },
//         ),
//         "MultiInstanceBehavior-One": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-MultiInstanceBehavior-One',
//                 name: "One",
//                 _classifier: "MultiInstanceBehavior",
//                 _enumeration: "MultiInstanceBehavior",
//                 litteral_name: "One",
//                 litteral_designation: "One",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#MultiInstanceBehavior",
//     table_name: "bpmn_20_multi_instance_behavior",
//     model_name: "MultiInstanceBehavior",
//     full_name: "bpmn_20_enumeration_multi_instance_behavior",
// }

