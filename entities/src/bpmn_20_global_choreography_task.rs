//! bpmn_20_class_global_choreography_task

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_global_choreography_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Choreography
    pub super_choreography: i64,
    /// COMPLEX FIELD : BPMN20-GlobalChoreographyTask-initiatingParticipantRef
    pub initiating_participant_ref: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE GlobalChoreographyTask need ONE Choreography
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography::Entity",
        from = "Column::SuperChoreography",
        to = "super::bpmn_20_choreography::Column::Id",
        on_delete = "Cascade"
    )]
    Choreography,
}

// SUPER : ONE GlobalChoreographyTask need ONE Choreography
impl Related<super::bpmn_20_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Choreography.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "GlobalChoreographyTask" (bpmn_20_class_global_choreography_task)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Participant__ (__ParticipantModel__) from A_initiatingParticipantRef_globalChoreographyTask
    ///   * one-to-many link : (1-1) __GlobalChoreographyTask__ need (0-inf) __Participant__)
    ///   * callable using find_with_related(__ParticipantModel__) from __GlobalChoreographyTask__
    /// 
    /// ## Direct Super :
    /// * __Choreography__ (__ChoreographyModel__)
    ///   * one-to-one link : one __GlobalChoreographyTask__ need one __Choreography__)
    ///   * callable using find_also_related(__ChoreographyModel__) from __GlobalChoreographyTask__
    ///   * saved in __super_choreography__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "GlobalChoreographyTask" (bpmn_20_class_global_choreography_task)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Participant__ (__ParticipantModel__) from A_initiatingParticipantRef_globalChoreographyTask
  * one-to-many link : (1-1) __GlobalChoreographyTask__ need (0-inf) __Participant__)
  * callable using find_with_related(__ParticipantModel__) from __GlobalChoreographyTask__

## Direct Super :
* __Choreography__ (__ChoreographyModel__)
  * one-to-one link : one __GlobalChoreographyTask__ need one __Choreography__)
  * callable using find_also_related(__ChoreographyModel__) from __GlobalChoreographyTask__
  * saved in __super_choreography__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "GlobalChoreographyTask",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "GlobalChoreographyTask",
//     is_abstract: false,
//     super_class: [
//         "Choreography",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "GlobalChoreographyTask-initiatingParticipantRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "GlobalChoreographyTask-initiatingParticipantRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "initiatingParticipantRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Participant",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_initiatingParticipantRef_globalChoreographyTask",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#GlobalChoreographyTask",
//     table_name: "bpmn_20_global_choreography_task",
//     model_name: "GlobalChoreographyTask",
//     full_name: "bpmn_20_class_global_choreography_task",
// }

