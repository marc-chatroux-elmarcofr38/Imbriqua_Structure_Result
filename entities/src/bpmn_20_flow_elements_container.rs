//! bpmn_20_class_flow_elements_container

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_flow_elements_container")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE FlowElementsContainer need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id"
    )]
    BaseElement,
    // SUPER : ONE Choreography need ONE FlowElementsContainer
    #[sea_orm(has_one = "super::bpmn_20_choreography::Entity")]
    Choreography,
    // SUPER : ONE Process need ONE FlowElementsContainer
    #[sea_orm(has_one = "super::bpmn_20_process::Entity")]
    Process,
    // SUPER : ONE SubChoreography need ONE FlowElementsContainer
    #[sea_orm(has_one = "super::bpmn_20_sub_choreography::Entity")]
    SubChoreography,
    // SUPER : ONE SubProcess need ONE FlowElementsContainer
    #[sea_orm(has_one = "super::bpmn_20_sub_process::Entity")]
    SubProcess,
}

// SUPER : ONE FlowElementsContainer need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// SUPER : ONE Choreography need ONE FlowElementsContainer
impl Related<super::bpmn_20_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Choreography.def()
    }
}

// SUPER : ONE Process need ONE FlowElementsContainer
impl Related<super::bpmn_20_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Process.def()
    }
}

// SUPER : ONE SubChoreography need ONE FlowElementsContainer
impl Related<super::bpmn_20_sub_choreography::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubChoreography.def()
    }
}

// SUPER : ONE SubProcess need ONE FlowElementsContainer
impl Related<super::bpmn_20_sub_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubProcess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "FlowElementsContainer",
//     name: "FlowElementsContainer",
//     is_abstract: true,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowElementsContainer-flowElements",
//                 name: "flowElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "FlowElement",
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
//                     "A_flowElements_container",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "FlowElementsContainer-laneSets",
//                 name: "laneSets",
//                 visibility: Public,
//                 simple_type: Some(
//                     "LaneSet",
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
//                     "A_laneSets_flowElementsContainer",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

