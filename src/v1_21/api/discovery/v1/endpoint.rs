// Generated from definition io.k8s.api.discovery.v1.Endpoint

/// Endpoint represents a single logical "backend" implementing a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Endpoint {
    /// addresses of this endpoint. The contents of this field are interpreted according to the corresponding EndpointSlice addressType field. Consumers must handle different types of addresses in the context of their own capabilities. This must contain at least one address but no more than 100.
    pub addresses: Vec<String>,

    /// conditions contains information about the current status of the endpoint.
    pub conditions: Option<crate::api::discovery::v1::EndpointConditions>,

    /// deprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.
    pub deprecated_topology: Option<std::collections::BTreeMap<String, String>>,

    /// hints contains information associated with how an endpoint should be consumed.
    pub hints: Option<crate::api::discovery::v1::EndpointHints>,

    /// hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.
    pub hostname: Option<String>,

    /// nodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node. This field can be enabled with the EndpointSliceNodeName feature gate.
    pub node_name: Option<String>,

    /// targetRef is a reference to a Kubernetes object that represents this endpoint.
    pub target_ref: Option<crate::api::core::v1::ObjectReference>,

    /// zone is the name of the Zone this endpoint exists in.
    pub zone: Option<String>,
}

impl<'de> serde::Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_addresses,
            Key_conditions,
            Key_deprecated_topology,
            Key_hints,
            Key_hostname,
            Key_node_name,
            Key_target_ref,
            Key_zone,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "addresses" => Field::Key_addresses,
                            "conditions" => Field::Key_conditions,
                            "deprecatedTopology" => Field::Key_deprecated_topology,
                            "hints" => Field::Key_hints,
                            "hostname" => Field::Key_hostname,
                            "nodeName" => Field::Key_node_name,
                            "targetRef" => Field::Key_target_ref,
                            "zone" => Field::Key_zone,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Endpoint;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Endpoint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<String>> = None;
                let mut value_conditions: Option<crate::api::discovery::v1::EndpointConditions> = None;
                let mut value_deprecated_topology: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_hints: Option<crate::api::discovery::v1::EndpointHints> = None;
                let mut value_hostname: Option<String> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_target_ref: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_zone: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_conditions => value_conditions = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_topology => value_deprecated_topology = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hints => value_hints = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ref => value_target_ref = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_zone => value_zone = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Endpoint {
                    addresses: value_addresses.ok_or_else(|| serde::de::Error::missing_field("addresses"))?,
                    conditions: value_conditions,
                    deprecated_topology: value_deprecated_topology,
                    hints: value_hints,
                    hostname: value_hostname,
                    node_name: value_node_name,
                    target_ref: value_target_ref,
                    zone: value_zone,
                })
            }
        }

        deserializer.deserialize_struct(
            "Endpoint",
            &[
                "addresses",
                "conditions",
                "deprecatedTopology",
                "hints",
                "hostname",
                "nodeName",
                "targetRef",
                "zone",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Endpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Endpoint",
            1 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.deprecated_topology.as_ref().map_or(0, |_| 1) +
            self.hints.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.target_ref.as_ref().map_or(0, |_| 1) +
            self.zone.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", &self.addresses)?;
        if let Some(value) = &self.conditions {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.deprecated_topology {
            serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedTopology", value)?;
        }
        if let Some(value) = &self.hints {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hints", value)?;
        }
        if let Some(value) = &self.hostname {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.node_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.target_ref {
            serde::ser::SerializeStruct::serialize_field(&mut state, "targetRef", value)?;
        }
        if let Some(value) = &self.zone {
            serde::ser::SerializeStruct::serialize_field(&mut state, "zone", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
