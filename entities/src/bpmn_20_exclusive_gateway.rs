//! bpmn_20_class_exclusive_gateway

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_exclusive_gateway")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : Gateway
    pub super_gateway: i64,
    /// COMPLEX FIELD : ExclusiveGateway-default
    pub default: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE ExclusiveGateway need ONE Gateway
    #[sea_orm(
        belongs_to = "super::bpmn_20_gateway::Entity",
        from = "Column::SuperGateway",
        to = "super::bpmn_20_gateway::Column::Id"
    )]
    Gateway,
}

// SUPER : ONE ExclusiveGateway need ONE Gateway
impl Related<super::bpmn_20_gateway::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Gateway.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFClass {
//     xmi_id: "ExclusiveGateway",
//     name: "ExclusiveGateway",
//     is_abstract: false,
//     super_class: [
//         "Gateway",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "ExclusiveGateway-default",
//                 name: "default",
//                 visibility: Public,
//                 simple_type: Some(
//                     "SequenceFlow",
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
//                     "A_default_exclusiveGateway",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

