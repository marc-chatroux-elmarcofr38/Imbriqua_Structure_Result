//! bpmn_20_class_operation

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_operation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : BaseElement
    pub super_base_element: i64,
    /// COMPLEX FIELD : Operation-implementationRef
    pub implementation_ref: Option<i64>,
    /// COMPLEX FIELD : Operation-inMessageRef
    pub in_message_ref: i64,
    /// COMPLEX FIELD : Operation-outMessageRef
    pub out_message_ref: Option<i64>,
    /// SIMPLE FIELD : Operation-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Operation need ONE BaseElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_base_element::Entity",
        from = "Column::SuperBaseElement",
        to = "super::bpmn_20_base_element::Column::Id",
        on_delete = "Cascade"
    )]
    BaseElement,
}

// SUPER : ONE Operation need ONE BaseElement
impl Related<super::bpmn_20_base_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BaseElement.def()
    }
}

// ManyToMany : with Error using A_errorRefs_operation
impl Related<super::bpmn_20_a_error_refs_operation::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_error_refs_operation::Relation::Error.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_error_refs_operation::Relation::Operation
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Operation" (bpmn_20_class_operation)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "Operation-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// ## Relation : One To Many :
    /// * __Message__ (__MessageModel__) from A_inMessageRef_operation
    ///   * one-to-many link : (1-1) __Operation__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __Operation__
    /// * __Interface__ (__InterfaceModel__) from A_operations_interface
    ///   * one-to-many link : (1-1) __Operation__ need (1-inf) __Interface__)
    ///   * callable using find_with_related(__InterfaceModel__) from __Operation__
    ///   * named interface in BPMN
    /// * __Message__ (__MessageModel__) from A_outMessageRef_operation
    ///   * one-to-many link : (0-1) __Operation__ need (0-inf) __Message__)
    ///   * callable using find_with_related(__MessageModel__) from __Operation__
    /// 
    /// ## Direct Super :
    /// * __BaseElement__ (__BaseElementModel__)
    ///   * one-to-one link : one __Operation__ need one __BaseElement__)
    ///   * callable using find_also_related(__BaseElementModel__) from __Operation__
    ///   * saved in __super_base_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Operation" (bpmn_20_class_operation)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "Operation-name")
  * type : __std::string::String__


## Relation : One To Many :
* __Message__ (__MessageModel__) from A_inMessageRef_operation
  * one-to-many link : (1-1) __Operation__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __Operation__
* __Interface__ (__InterfaceModel__) from A_operations_interface
  * one-to-many link : (1-1) __Operation__ need (1-inf) __Interface__)
  * callable using find_with_related(__InterfaceModel__) from __Operation__
  * named interface in BPMN
* __Message__ (__MessageModel__) from A_outMessageRef_operation
  * one-to-many link : (0-1) __Operation__ need (0-inf) __Message__)
  * callable using find_with_related(__MessageModel__) from __Operation__

## Direct Super :
* __BaseElement__ (__BaseElementModel__)
  * one-to-one link : one __Operation__ need one __BaseElement__)
  * callable using find_also_related(__BaseElementModel__) from __Operation__
  * saved in __super_base_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Operation",
//     name: "Operation",
//     is_abstract: false,
//     super_class: [
//         "BaseElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "Operation-errorRefs": Property(
//             CMOFProperty {
//                 xmi_id: "Operation-errorRefs",
//                 name: "errorRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Error",
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
//                 owning_association: "",
//                 association: Some(
//                     "A_errorRefs_operation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Operation-implementationRef": Property(
//             CMOFProperty {
//                 xmi_id: "Operation-implementationRef",
//                 name: "implementationRef",
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
//                 association: None,
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Operation-inMessageRef": Property(
//             CMOFProperty {
//                 xmi_id: "Operation-inMessageRef",
//                 name: "inMessageRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                 association: Some(
//                     "A_inMessageRef_operation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "Operation-name": Property(
//             CMOFProperty {
//                 xmi_id: "Operation-name",
//                 name: "name",
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
//         "Operation-outMessageRef": Property(
//             CMOFProperty {
//                 xmi_id: "Operation-outMessageRef",
//                 name: "outMessageRef",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Message",
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
//                     "A_outMessageRef_operation",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
// }

