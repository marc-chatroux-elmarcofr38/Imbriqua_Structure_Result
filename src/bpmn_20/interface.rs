//! interface
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Interface (Class : Interface)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Interface",
///     name: "Interface",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-operations",
///                 name: "operations",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Operation",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_operations_interface",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Interface-implementationRef",
///                 name: "implementationRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Interface<'a> {
    #[builder(setter(into))]
    pub name: dc::String,
    #[builder(setter(into))]
    pub operations: Vec<&'a Operation<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub implementation_ref: Option<i8>,
}

