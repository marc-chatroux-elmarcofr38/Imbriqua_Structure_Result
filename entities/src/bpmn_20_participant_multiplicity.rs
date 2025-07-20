//! bpmn_20_class_participant_multiplicity

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_participant_multiplicity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : BPMN20-ParticipantMultiplicity-maximum
    #[sea_orm(default_value = "1")]
    pub maximum: Option<std::primitive::u64>,
    /// SIMPLE FIELD : BPMN20-ParticipantMultiplicity-minimum
    #[sea_orm(default_value = "0")]
    pub minimum: std::primitive::u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "ParticipantMultiplicity" (bpmn_20_class_participant_multiplicity)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __maximum__ (xmi_id : "BPMN20-ParticipantMultiplicity-maximum")
    ///   * type : __Option<std::primitive::u64>__
    ///   * default : "1"
    /// * __minimum__ (xmi_id : "BPMN20-ParticipantMultiplicity-minimum")
    ///   * type : __std::primitive::u64__
    ///   * default : "0"
    /// 
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __Participant__ (__ParticipantModel__) from A_participantMultiplicity_participant
    ///   * one-to-one link : (0-1) __Participant__ need (1-1) __ParticipantMultiplicity__)
    ///   * callable using find_also_related(__ParticipantMultiplicityModel__) from __Participant__
    ///   * saved in __participant_multiplicity__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "ParticipantMultiplicity" (bpmn_20_class_participant_multiplicity)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __maximum__ (xmi_id : "BPMN20-ParticipantMultiplicity-maximum")
  * type : __Option<std::primitive::u64>__
  * default : "1"
* __minimum__ (xmi_id : "BPMN20-ParticipantMultiplicity-minimum")
  * type : __std::primitive::u64__
  * default : "0"



## Reverse One To One :
* __Participant__ (__ParticipantModel__) from A_participantMultiplicity_participant
  * one-to-one link : (0-1) __Participant__ need (1-1) __ParticipantMultiplicity__)
  * callable using find_also_related(__ParticipantMultiplicityModel__) from __Participant__
  * saved in __participant_multiplicity__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-ParticipantMultiplicity" (loaded : false)",
//     name: "ParticipantMultiplicity",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "-ParticipantMultiplicity-maximum": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ParticipantMultiplicity-maximum" (loaded : false)",
//                 name: "maximum",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of "DC-Integer" (loaded : false)",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "1",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-ParticipantMultiplicity-minimum": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-ParticipantMultiplicity-minimum" (loaded : false)",
//                 name: "minimum",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of "DC-Integer" (loaded : false)",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "0",
//                 ),
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#ParticipantMultiplicity",
//     table_name: "bpmn_20_participant_multiplicity",
//     model_name: "ParticipantMultiplicity",
//     full_name: "bpmn_20_class_participant_multiplicity",
// }

