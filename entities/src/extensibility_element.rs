//! extensibility_class_element

use crate::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "extensibility_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SIMPLE FIELD : Element-Content
    pub content: JsonContent,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

// ManyToMany : with Relationship using A_sources_relationship
impl Related<super::bpmn_20_a_sources_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_sources_relationship::Relation::Relationship.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_sources_relationship::Relation::Element
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with Relationship using A_targets_relationship
impl Related<super::bpmn_20_a_targets_relationship::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_targets_relationship::Relation::Relationship.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_targets_relationship::Relation::Element
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Element" (extensibility_class_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __content__ (xmi_id : "Element-Content")
    ///   * type : __JsonContent__
    /// 
    /// 
    /// 
    /// ## Reverse One To One :
    /// * __ExtensionAttributeValue__ (__ExtensionAttributeValueModel__) from A_value_extensionAttributeValue
    ///   * one-to-one link : (0-1) __ExtensionAttributeValue__ need (1-1) __Element__)
    ///   * callable using find_also_related(__ElementModel__) from __ExtensionAttributeValue__
    ///   * saved in __value__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Element" (extensibility_class_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __content__ (xmi_id : "Element-Content")
  * type : __JsonContent__



## Reverse One To One :
* __ExtensionAttributeValue__ (__ExtensionAttributeValueModel__) from A_value_extensionAttributeValue
  * one-to-one link : (0-1) __ExtensionAttributeValue__ need (1-1) __Element__)
  * callable using find_also_related(__ElementModel__) from __ExtensionAttributeValue__
  * saved in __value__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Element",
//     name: "Element",
//     is_abstract: false,
//     super_class: [],
//     super_class_link: [],
//     owned_attribute: {
//         "Element-Content": Property(
//             CMOFProperty {
//                 xmi_id: "Element-Content",
//                 name: "Content",
//                 visibility: Public,
//                 simple_type: Some(
//                     "JsonContent",
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
//     },
//     owned_rule: {},
// }

