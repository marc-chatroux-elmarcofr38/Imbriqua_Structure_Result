//! bpmn_20_association_a_sources_relationship

use sea_orm::entity::prelude::*;
    
#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_a_sources_relationship")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub element_a_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub relationship_b_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::extensibility_element::Entity",
        from = "Column::ElementAId",
        to = "super::extensibility_element::Column::Id"
    )]
    Element,
    #[sea_orm(
        belongs_to = "super::bpmn_20_relationship::Entity",
        from = "Column::RelationshipBId",
        to = "super::bpmn_20_relationship::Column::Id"
    )]
    Relationship,
}

impl ActiveModelBehavior for ActiveModel {}

// RAW :
// CMOFAssociation {
//     xmi_id: "A_sources_relationship",
//     name: "A_sources_relationship",
//     visibility: Private,
//     member_end: (
//         "Relationship-sources",
//         "A_sources_relationship-relationship",
//     ),
//     owned_end: {
//         "A_sources_relationship-relationship": Property(
//             CMOFProperty {
//                 xmi_id: "A_sources_relationship-relationship",
//                 name: "relationship",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Relationship",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Infinity,
//                 default: None,
//                 is_read_only: false,
//                 is_composite: false,
//                 is_unique: false,
//                 is_ordered: false,
//                 is_abstract: None,
//                 is_derived: false,
//                 is_derived_union: false,
//                 subsetted_property: None,
//                 owning_association: "A_sources_relationship",
//                 association: Some(
//                     "A_sources_relationship",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     is_derived: false,
// }

