//! bpmn_20_enumeration_association_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssociationDirection {
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-Both
    #[sea_orm(string_value = "Both")]
    Both,
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : BPMN20-AssociationDirection-One
    #[sea_orm(string_value = "One")]
    One,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AssociationDirection',
//     name: "AssociationDirection",
//     owned_attribute: {
//         "AssociationDirection-Both": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AssociationDirection-Both',
//                 name: "Both",
//                 _classifier: "AssociationDirection",
//                 _enumeration: "AssociationDirection",
//                 litteral_name: "Both",
//                 litteral_designation: "Both",
//             },
//         ),
//         "AssociationDirection-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AssociationDirection-None',
//                 name: "None",
//                 _classifier: "AssociationDirection",
//                 _enumeration: "AssociationDirection",
//                 litteral_name: "None",
//                 litteral_designation: "None",
//             },
//         ),
//         "AssociationDirection-One": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-AssociationDirection-One',
//                 name: "One",
//                 _classifier: "AssociationDirection",
//                 _enumeration: "AssociationDirection",
//                 litteral_name: "One",
//                 litteral_designation: "One",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#AssociationDirection",
//     table_name: "bpmn_20_association_direction",
//     model_name: "AssociationDirection",
//     full_name: "bpmn_20_enumeration_association_direction",
// }

