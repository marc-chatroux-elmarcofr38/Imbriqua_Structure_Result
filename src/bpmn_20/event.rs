//! event

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Event (Class : Event)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Event",
///     name: "Event",
///     is_abstract: true,
///     super_class: Some(
///         "FlowNode InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Event-properties",
///                 name: "properties",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Property",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
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
///                 association: "A_properties_event",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Event<'a> {
    #[builder(setter(into, strip_option), default)]
    pub properties: Option<Vec<&'a Property<'a>>>,
}

