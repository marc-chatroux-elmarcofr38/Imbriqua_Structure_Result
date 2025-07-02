//! bpmn_20_class_event_based_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_event_based_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// SUPER FIELD : Gateway
    pub super_gateway: i32,
    /// SIMPLE FIELD : EventBasedGateway-instantiate
    #[sea_orm(default_value = "false")]
    pub instantiate: std::primitive::bool,
    /// SIMPLE FIELD : EventBasedGateway-eventGatewayType
    pub event_gateway_type: EventBasedGatewayType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE EventBasedGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id"
    )]
    Gateway,
}

// SUPER : ONE EventBasedGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "EventBasedGateway",
//     name: "EventBasedGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "EventBasedGateway-instantiate",
//                 name: "instantiate",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#Boolean",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
//                 upper: Finite(
//                     1,
//                 ),
//                 default: Some(
//                     "false",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "EventBasedGateway-eventGatewayType",
//                 name: "eventGatewayType",
//                 visibility: Public,
//                 simple_type: Some(
//                     "EventBasedGatewayType",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

