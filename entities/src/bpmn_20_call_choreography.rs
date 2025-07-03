//! bpmn_20_class_call_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_call_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ChoreographyActivity
    pub super_choreography_activity: i64,
    /// COMPLEX FIELD : CallChoreography-calledChoreographyRef
    pub called_choreography_ref: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallChoreography need ONE ChoreographyActivity
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::SuperChoreographyActivity",
        to = "super::bpmn_20_choreography_activity::Column::Id"
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

// RAW :
// CMOFClass {
//     xmi_id: "CallChoreography",
//     name: "CallChoreography",
//     is_abstract: false,
//     super_class: [
//         "ChoreographyActivity",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "CallChoreography-calledChoreographyRef",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "CallChoreography-participantAssociations",
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
//     ],
//     owned_rule: [],
// }

