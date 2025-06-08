//! signal_event_definition

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of SignalEventDefinition (Class : SignalEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SignalEventDefinition",
///     name: "SignalEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SignalEventDefinition-signalRef",
///                 name: "signalRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Signal",
///                 ),
///                 complex_type: None,
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
///                 association: "A_signalRef_signalEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct SignalEventDefinition<'a> {
    #[builder(setter(into, strip_option), default)]
    pub signal_ref: Option<&'a Signal<'a>>,
}

