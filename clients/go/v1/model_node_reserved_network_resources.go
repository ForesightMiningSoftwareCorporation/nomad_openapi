/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.1.4
 * Contact: support@hashicorp.com
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package client

import (
	"encoding/json"
)

// NodeReservedNetworkResources struct for NodeReservedNetworkResources
type NodeReservedNetworkResources struct {
	ReservedHostPorts *string `json:"ReservedHostPorts,omitempty"`
}

// NewNodeReservedNetworkResources instantiates a new NodeReservedNetworkResources object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewNodeReservedNetworkResources() *NodeReservedNetworkResources {
	this := NodeReservedNetworkResources{}
	return &this
}

// NewNodeReservedNetworkResourcesWithDefaults instantiates a new NodeReservedNetworkResources object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewNodeReservedNetworkResourcesWithDefaults() *NodeReservedNetworkResources {
	this := NodeReservedNetworkResources{}
	return &this
}

// GetReservedHostPorts returns the ReservedHostPorts field value if set, zero value otherwise.
func (o *NodeReservedNetworkResources) GetReservedHostPorts() string {
	if o == nil || o.ReservedHostPorts == nil {
		var ret string
		return ret
	}
	return *o.ReservedHostPorts
}

// GetReservedHostPortsOk returns a tuple with the ReservedHostPorts field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *NodeReservedNetworkResources) GetReservedHostPortsOk() (*string, bool) {
	if o == nil || o.ReservedHostPorts == nil {
		return nil, false
	}
	return o.ReservedHostPorts, true
}

// HasReservedHostPorts returns a boolean if a field has been set.
func (o *NodeReservedNetworkResources) HasReservedHostPorts() bool {
	if o != nil && o.ReservedHostPorts != nil {
		return true
	}

	return false
}

// SetReservedHostPorts gets a reference to the given string and assigns it to the ReservedHostPorts field.
func (o *NodeReservedNetworkResources) SetReservedHostPorts(v string) {
	o.ReservedHostPorts = &v
}

func (o NodeReservedNetworkResources) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.ReservedHostPorts != nil {
		toSerialize["ReservedHostPorts"] = o.ReservedHostPorts
	}
	return json.Marshal(toSerialize)
}

type NullableNodeReservedNetworkResources struct {
	value *NodeReservedNetworkResources
	isSet bool
}

func (v NullableNodeReservedNetworkResources) Get() *NodeReservedNetworkResources {
	return v.value
}

func (v *NullableNodeReservedNetworkResources) Set(val *NodeReservedNetworkResources) {
	v.value = val
	v.isSet = true
}

func (v NullableNodeReservedNetworkResources) IsSet() bool {
	return v.isSet
}

func (v *NullableNodeReservedNetworkResources) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableNodeReservedNetworkResources(val *NodeReservedNetworkResources) *NullableNodeReservedNetworkResources {
	return &NullableNodeReservedNetworkResources{value: val, isSet: true}
}

func (v NullableNodeReservedNetworkResources) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableNodeReservedNetworkResources) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


