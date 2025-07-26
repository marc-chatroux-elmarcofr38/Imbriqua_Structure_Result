//! bpmn_20_class_inclusive_gateway

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_inclusive_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : SuperGateway
    pub super_gateway: i64,
    /// COMPLEX FIELD : BPMN20-InclusiveGateway-default
    pub default: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE InclusiveGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id",
        on_delete = "Cascade"
    )]
    Gateway,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {

    pub fn help(&self) -> &str {
    r#""#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-InclusiveGateway',
//     name: "InclusiveGateway",
//     is_abstract: false,
//     super_class: [
//         "Loaded XMIIdReference RefCell of 'BPMN20-Gateway',
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "InclusiveGateway-default": Property(
//             CMOFProperty {
//                 xmi_id: "Complete XMIIdLocalReference RefCell of 'BPMN20-InclusiveGateway-default',
//                 name: "default",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-SequenceFlow',
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
//                 owning_association: None,
//                 association: Some(
//                     "Loaded XMIIdReference RefCell of 'BPMN20-A_default_inclusiveGateway',
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#InclusiveGateway",
//     table_name: "bpmn_20_inclusive_gateway",
//     model_name: "InclusiveGateway",
//     full_name: "bpmn_20_class_inclusive_gateway",
//     reverse_super: RefCell {
//         value: [],
//     },
// }

