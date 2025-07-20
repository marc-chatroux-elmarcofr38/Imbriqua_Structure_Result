//! bpmn_20_class_callable_element

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_callable_element")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-CallableElement-ioSpecification
    pub io_specification: Option<i64>,
    /// SIMPLE FIELD : BPMN20-CallableElement-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE CallableElement need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
    // SUPER : ONE GlobalTask need ONE CallableElement
    #[sea_orm(has_one = "super::bpmn_20_global_task::Entity")]
    GlobalTask,
    // SUPER : ONE Process need ONE CallableElement
    #[sea_orm(has_one = "super::bpmn_20_process::Entity")]
    Process,
}

// SUPER : ONE CallableElement need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// SUPER : ONE GlobalTask need ONE CallableElement
impl Related<super::bpmn_20_global_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GlobalTask.def()
    }
}

// SUPER : ONE Process need ONE CallableElement
impl Related<super::bpmn_20_process::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Process.def()
    }
}

// ManyToMany : with Interface using A_supportedInterfaceRefs_callableElements
impl Related<super::bpmn_20_a_supported_interface_refs_callable_elements::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_supported_interface_refs_callable_elements::Relation::Interface.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_supported_interface_refs_callable_elements::Relation::CallableElement
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "CallableElement" (bpmn_20_class_callable_element)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-CallableElement-name")
    ///   * type : __std::string::String__
    /// 
    /// ## Direct One To One :
    /// * __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_ioSpecification_callableElement
    ///   * one-to-one link : (0-1) __CallableElement__ need (0-1) __InputOutputSpecification__)
    ///   * callable using find_also_related(__InputOutputSpecificationModel__) from __CallableElement__
    ///   * saved in __io_specification__ field as foreing key
    /// 
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __CallableElement__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __CallableElement__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// ## Reverse Super :
    /// * __GlobalTask__ (__GlobalTaskModel__)
    ///   * one-to-one link (reverse) : one __GlobalTask__ need one __CallableElement__)
    ///   * callable using find_also_related(__CallableElementModel__) from __GlobalTask__
    ///   * saved in __super_callable_element__ field as foreing key in __GlobalTaskModel__
    /// * __Process__ (__ProcessModel__)
    ///   * one-to-one link (reverse) : one __Process__ need one __CallableElement__)
    ///   * callable using find_also_related(__CallableElementModel__) from __Process__
    ///   * saved in __super_callable_element__ field as foreing key in __ProcessModel__
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "CallableElement" (bpmn_20_class_callable_element)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-CallableElement-name")
  * type : __std::string::String__

## Direct One To One :
* __InputOutputSpecification__ (__InputOutputSpecificationModel__) from A_ioSpecification_callableElement
  * one-to-one link : (0-1) __CallableElement__ need (0-1) __InputOutputSpecification__)
  * callable using find_also_related(__InputOutputSpecificationModel__) from __CallableElement__
  * saved in __io_specification__ field as foreing key


## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __CallableElement__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __CallableElement__
  * saved in __super_root_element__ field as foreing key

## Reverse Super :
* __GlobalTask__ (__GlobalTaskModel__)
  * one-to-one link (reverse) : one __GlobalTask__ need one __CallableElement__)
  * callable using find_also_related(__CallableElementModel__) from __GlobalTask__
  * saved in __super_callable_element__ field as foreing key in __GlobalTaskModel__
* __Process__ (__ProcessModel__)
  * one-to-one link (reverse) : one __Process__ need one __CallableElement__)
  * callable using find_also_related(__CallableElementModel__) from __Process__
  * saved in __super_callable_element__ field as foreing key in __ProcessModel__

"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-CallableElement" (loaded : false)",
//     name: "CallableElement",
//     is_abstract: true,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-CallableElement-ioBinding": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CallableElement-ioBinding" (loaded : false)",
//                 name: "ioBinding",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputOutputBinding",
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
//                     "A_ioBinding_callableElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CallableElement-ioSpecification": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CallableElement-ioSpecification" (loaded : false)",
//                 name: "ioSpecification",
//                 visibility: Public,
//                 simple_type: Some(
//                     "InputOutputSpecification",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 0,
//                 upper: Finite(
//                     1,
//                 ),
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
//                     "A_ioSpecification_callableElement",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//         "-CallableElement-name": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CallableElement-name" (loaded : false)",
//                 name: "name",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefPrimitiveType(
//                         HRefPrimitiveType {
//                             href: "Weak ref of "DC-String" (loaded : false)",
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
//         "-CallableElement-supportedInterfaceRefs": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-CallableElement-supportedInterfaceRefs" (loaded : false)",
//                 name: "supportedInterfaceRefs",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Interface",
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
//                     "A_supportedInterfaceRefs_callableElements",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#CallableElement",
//     table_name: "bpmn_20_callable_element",
//     model_name: "CallableElement",
//     full_name: "bpmn_20_class_callable_element",
// }

