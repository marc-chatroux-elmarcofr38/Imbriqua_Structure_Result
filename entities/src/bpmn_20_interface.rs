//! bpmn_20_class_interface

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bpmn_20_interface")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// SUPER FIELD : RootElement
    pub super_root_element: i64,
    /// COMPLEX FIELD : BPMN20-Interface-implementationRef
    pub implementation_ref: Option<i64>,
    /// SIMPLE FIELD : BPMN20-Interface-name
    pub name: std::string::String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // SUPER : ONE Interface need ONE RootElement
    #[sea_orm(
        belongs_to = "super::bpmn_20_root_element::Entity",
        from = "Column::SuperRootElement",
        to = "super::bpmn_20_root_element::Column::Id",
        on_delete = "Cascade"
    )]
    RootElement,
}

// SUPER : ONE Interface need ONE RootElement
impl Related<super::bpmn_20_root_element::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RootElement.def()
    }
}

// ManyToMany : with Participant using A_interfaceRefs_participant
impl Related<super::bpmn_20_a_interface_refs_participant::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_interface_refs_participant::Relation::Participant.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_interface_refs_participant::Relation::Interface
                .def()
                .rev(),
        )
    }
}

// ManyToMany : with CallableElement using A_supportedInterfaceRefs_callableElements
impl Related<super::bpmn_20_a_supported_interface_refs_callable_elements::Entity> for Entity {
    fn to() -> RelationDef {
        super::bpmn_20_a_supported_interface_refs_callable_elements::Relation::CallableElement.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::bpmn_20_a_supported_interface_refs_callable_elements::Relation::Interface
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    /// # Help document for "Interface" (bpmn_20_class_interface)
    /// 
    /// ## Common fields :
    /// * __id__ (sea_orm only)
    ///   * type : __i64__
    /// 
    /// ## Simple fields :
    /// * __name__ (xmi_id : "BPMN20-Interface-name")
    ///   * type : __std::string::String__
    /// 
    /// 
    /// 
    /// ## Direct Super :
    /// * __RootElement__ (__RootElementModel__)
    ///   * one-to-one link : one __Interface__ need one __RootElement__)
    ///   * callable using find_also_related(__RootElementModel__) from __Interface__
    ///   * saved in __super_root_element__ field as foreing key
    /// 
    /// 

    pub fn help(&self) -> &str {
    r#"# Help document for "Interface" (bpmn_20_class_interface)

## Common fields :
* __id__ (sea_orm only)
  * type : __i64__

## Simple fields :
* __name__ (xmi_id : "BPMN20-Interface-name")
  * type : __std::string::String__



## Direct Super :
* __RootElement__ (__RootElementModel__)
  * one-to-one link : one __Interface__ need one __RootElement__)
  * callable using find_also_related(__RootElementModel__) from __Interface__
  * saved in __super_root_element__ field as foreing key


"#
    }
}

// RAW :
// CMOFClass {
//     xmi_id: "Weak ref of "BPMN20-Interface" (loaded : false)",
//     name: "Interface",
//     is_abstract: false,
//     super_class: [
//         "RootElement",
//     ],
//     super_class_link: [],
//     owned_attribute: {
//         "-Interface-implementationRef": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-Interface-implementationRef" (loaded : false)",
//                 name: "implementationRef",
//                 visibility: Public,
//                 simple_type: None,
//                 complex_type: Some(
//                     HRefClass(
//                         HRefClass {
//                             href: "Weak ref of "Extensibilty-Element" (loaded : false)",
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
//         "-Interface-name": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-Interface-name" (loaded : false)",
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
//         "-Interface-operations": Property(
//             CMOFProperty {
//                 xmi_id: "Weak ref of "BPMN20-Interface-operations" (loaded : false)",
//                 name: "operations",
//                 visibility: Public,
//                 simple_type: Some(
//                     "Operation",
//                 ),
//                 complex_type: None,
//                 datatype: None,
//                 lower: 1,
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
//                     "A_operations_interface",
//                 ),
//                 redefined_property_link: None,
//                 subsetted_property_link: None,
//             },
//         ),
//     },
//     owned_rule: {},
//     technical_name: "BPMN20.cmof#Interface",
//     table_name: "bpmn_20_interface",
//     model_name: "Interface",
//     full_name: "bpmn_20_class_interface",
// }

