//! bpmn_20_class_flow_elements_container

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_flow_elements_container")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperBaseElement
    pub super_base_element: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE FlowElementsContainer need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
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

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-FlowElementsContainer',
//     name: "FlowElementsContainer",
//     is_abstract: true,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-BaseElement',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "FlowElementsContainer-flowElements": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-FlowElementsContainer-flowElements',
//                 name: "flowElements",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-FlowElement',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_flowElements_container',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "FlowElementsContainer-laneSets": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-FlowElementsContainer-laneSets',
//                 name: "laneSets",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-LaneSet',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_laneSets_flowElementsContainer',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#FlowElementsContainer",
//     table_name: "bpmn_20_flow_elements_container",
//     model_name: "FlowElementsContainer",
//     full_name: "bpmn_20_class_flow_elements_container",
//     reverse_super: RefCell {
//         value: [
//             (Weak),
//             (Weak),
//             (Weak),
//             (Weak),
//         ],
//     },
// }

