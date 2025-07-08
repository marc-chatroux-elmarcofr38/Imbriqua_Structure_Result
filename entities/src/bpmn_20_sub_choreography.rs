//! bpmn_20_class_sub_choreography

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_sub_choreography")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : ChoreographyActivity
    pub super_choreography_activity: i64,
    /// SUPER FIELD : FlowElementsContainer
    pub super_flow_elements_container: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE SubChoreography need ONE ChoreographyActivity
    #[sea_orm(
        belongs_to = "super::bpmn_20_choreography_activity::Entity",
        from = "Column::SuperChoreographyActivity",
        to = "super::bpmn_20_choreography_activity::Column::Id",
        on_delete = "Cascade"
    )]
    ChoreographyActivity,
    // SUPER : ONE SubChoreography need ONE FlowElementsContainer
    #[sea_orm(
        belongs_to = "super::bpmn_20_flow_elements_container::Entity",
        from = "Column::SuperFlowElementsContainer",
        to = "super::bpmn_20_flow_elements_container::Column::Id",
        on_delete = "Cascade"
    )]
    FlowElementsContainer,
}

// SUPER : ONE SubChoreography need ONE ChoreographyActivity
impl Related<super::bpmn_20_choreography_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChoreographyActivity.def()
    }
}

// SUPER : ONE SubChoreography need ONE FlowElementsContainer
impl Related<super::bpmn_20_flow_elements_container::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowElementsContainer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "SubChoreography",
//     name: "SubChoreography",
//     is_abstract: false,
//     super_class: [
//         "ChoreographyActivity",
//         "FlowElementsContainer",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "SubChoreography-artifacts",
//                 name: "artifacts",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Artifact",
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
//                     "A_artifacts_subChoreography",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

