//! EventBasedGateway
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of EventBasedGateway (Class : EventBasedGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EventBasedGateway",
///     name: "EventBasedGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "EventBasedGateway-instantiate",
///                 name: "instantiate",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
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
///                 xmi_id: "EventBasedGateway-eventGatewayType",
///                 name: "eventGatewayType",
///                 visibility: Public,
///                 simple_type: Some(
///                     "EventBasedGatewayType",
///                 ),
///                 complex_type: None,
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
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct EventBasedGateway {
    #[builder(setter(into), default = "false")]
    pub instantiate: dc::Boolean,
    #[builder(setter(into))]
    pub event_gateway_type: EventBasedGatewayType,
}

