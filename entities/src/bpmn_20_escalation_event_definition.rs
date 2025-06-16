//! bpmn_20_class_escalation_event_definition

use crate::*;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "bpmn_20_escalation_event_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "EscalationEventDefinition",
//     name: "EscalationEventDefinition",
//     is_abstract: false,
//     super_class: Some(
//         "EventDefinition",
//     ),
//     super_class_link: None,
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "EscalationEventDefinition-escalationRef",
//                 name: "escalationRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Escalation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_escalationRef_escalationEventDefinition",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

