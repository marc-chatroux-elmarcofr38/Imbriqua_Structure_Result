//! bpmn_20_class_call_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ChoreographyActivity
    pub super_choreography_activity: i64,
    /// COMPLEX FIELD : BPMN20-CallChoreography-calledChoreographyRef
    pub called_choreography_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallChoreography need ONE ChoreographyActivity
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::SuperChoreographyActivity",
        to = "super::bpmn_20_choreography_activity::Column::Id",
        on_delete = "Cascade"
    )]
    ChoreographyActivity,
}

// SUPER : ONE CallChoreography need ONE ChoreographyActivity
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CallChoreography" (bpmn_20_class_call_choreography)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Choreography__ (__ChoreographyModel__) from A_calledChoreographyRef_callChoreographyActivity
    ///   * one-to-many link : (0-1) __CallChoreography__ need (0-inf) __Choreography__)
    ///   * callable using find_with_related(__ChoreographyModel__) from __CallChoreography__
    /// 
    /// ## Direct Super :
    /// * __ChoreographyActivity__ (__ChoreographyActivityModel__)
    ///   * one-to-one link : one __CallChoreography__ need one __ChoreographyActivity__)
    ///   * callable using find_also_related(__ChoreographyActivityModel__) from __CallChoreography__
    ///   * saved in __super_choreography_activity__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CallChoreography" (bpmn_20_class_call_choreography)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__



## Relation : One To Many :
* __Choreography__ (__ChoreographyModel__) from A_calledChoreographyRef_callChoreographyActivity
  * one-to-many link : (0-1) __CallChoreography__ need (0-inf) __Choreography__)
  * callable using find_with_related(__ChoreographyModel__) from __CallChoreography__

## Direct Super :
* __ChoreographyActivity__ (__ChoreographyActivityModel__)
  * one-to-one link : one __CallChoreography__ need one __ChoreographyActivity__)
  * callable using find_also_related(__ChoreographyActivityModel__) from __CallChoreography__
  * saved in __super_choreography_activity__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: XMIIdLocalReference {
//         object_id: "CallChoreography",
//         package_id: "BPMN20",
//         is_set: true,
//     },
//     name: "CallChoreography",
//     is_abstract: false,
//     super_class: [
//         "ChoreographyActivity",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "CallChoreography-calledChoreographyRef": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "CallChoreography-calledChoreographyRef",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "calledChoreographyRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Choreography",
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
//                     "A_calledChoreographyRef_callChoreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "CallChoreography-participantAssociations": Property(
//             CMOFProperty {
//                 xmi_id: XMIIdLocalReference {
//                     object_id: "CallChoreography-participantAssociations",
//                     package_id: "BPMN20",
//                     is_set: true,
//                 },
//                 name: "participantAssociations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "ParticipantAssociation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: true,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "",
//                 association: Some(
//                     "A_participantAssociations_callChoreographyActivity",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CallChoreography",
//     table_name: "bpmn_20_call_choreography",
//     model_name: "CallChoreography",
//     full_name: "bpmn_20_class_call_choreography",
// }

