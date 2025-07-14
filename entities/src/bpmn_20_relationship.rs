//! bpmn_20_class_relationship

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_relationship")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// SIMPLE FIELD : Relationship-type
    pub r#type: std::string::String,
    /// SIMPLE FIELD : Relationship-direction
    pub direction: RelationshipDirection,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Relationship need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Relationship need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with Element using A_sources_relationship
impl Related<super::bpmn_20_a_sources_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_sources_relationship::Relation::Element.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_sources_relationship::Relation::Relationship
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with Element using A_targets_relationship
impl Related<super::bpmn_20_a_targets_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_targets_relationship::Relation::Element.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_targets_relationship::Relation::Relationship
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Relationship" (bpmn_20_class_relationship)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __r#type__ (xmi_id : "Relationship-type")
    ///   * type : __std::string::String__
    /// * __direction__ (xmi_id : "Relationship-direction")
    ///   * type : __RelationshipDirection__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Definitions__ (__DefinitionsModel__) from A_relationships_definition
    ///   * one-to-many link : (1-1) __Relationship__ need (0-inf) __Definitions__)
    ///   * callable using find_with_related(__DefinitionsModel__) from __Relationship__
    ///   * named definition in BPMN
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Relationship__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Relationship__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Relationship" (bpmn_20_class_relationship)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __r#type__ (xmi_id : "Relationship-type")
  * type : __std::string::String__
* __direction__ (xmi_id : "Relationship-direction")
  * type : __RelationshipDirection__


## Relation : One To Many :
* __Definitions__ (__DefinitionsModel__) from A_relationships_definition
  * one-to-many link : (1-1) __Relationship__ need (0-inf) __Definitions__)
  * callable using find_with_related(__DefinitionsModel__) from __Relationship__
  * named definition in BPMN

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Relationship__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Relationship__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Relationship",
//     name: "Relationship",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: [
//         Property(
//             CMOFProperty {
//                 xmi_id: "Relationship-type",
//                 name: "r#type",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     PrimitiveTypeLink(
//                         PrimitiveTypeLink {
//                             href: "DC.cmof#String",
//                         },
//                     ),
//                 ),
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Relationship-direction",
//                 name: "direction",
//                 visibility: Public,
//                 simple_type: Some(
//                     "RelationshipDirection",
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
//         Property(
//             CMOFProperty {
//                 xmi_id: "Relationship-sources",
//                 name: "sources",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "",
//                 association: Some(
//                     "A_sources_relationship",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         Property(
//             CMOFProperty {
//                 xmi_id: "Relationship-targets",
//                 name: "targets",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     ClassLink(
//                         ClassLink {
//                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
//                         },
//                     ),
//                 ),
//                 datatype: None,
//                 lower: 1,
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
//                 owning_association: "",
//                 association: Some(
//                     "A_targets_relationship",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     ],
//     owned_rule: [],
// }

