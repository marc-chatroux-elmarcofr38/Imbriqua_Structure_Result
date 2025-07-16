//! bpmn_20_enumeration_association_direction

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AssociationDirection {
    /// ENUMERATION LITERAL : AssociationDirection-Both
    #[sea_orm(string_value = "Both")]
    Both,
    /// ENUMERATION LITERAL : AssociationDirection-None
    #[default]
    #[sea_orm(string_value = "None")]
    None,
    /// ENUMERATION LITERAL : AssociationDirection-One
    #[sea_orm(string_value = "One")]
    One,
}


// RAW :
// CMOFEnumeration {
//     xmi_id: "AssociationDirection",
//     name: "AssociationDirection",
//     owned_attribute: {
//         "AssociationDirection-Both": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-Both",
//                 name: "Both",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         "AssociationDirection-None": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-None",
//                 name: "None",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//         "AssociationDirection-One": EnumerationLiteral(
//             CMOFEnumerationLiteral {
//                 xmi_id: "AssociationDirection-One",
//                 name: "One",
//                 classifier: "AssociationDirection",
//                 enumeration: "AssociationDirection",
//             },
//         ),
//     },
//     technical_name: "BPMN20.cmof#AssociationDirection",
//     table_name: "bpmn_20_association_direction",
//     model_name: "AssociationDirection",
//     full_name: "bpmn_20_enumeration_association_direction",
// }

