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

// CSITopologyRequest struct for CSITopologyRequest
type CSITopologyRequest struct {
	Preferred *[]CSITopology `json:"Preferred,omitempty"`
	Required *[]CSITopology `json:"Required,omitempty"`
}

// NewCSITopologyRequest instantiates a new CSITopologyRequest object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewCSITopologyRequest() *CSITopologyRequest {
	this := CSITopologyRequest{}
	return &this
}

// NewCSITopologyRequestWithDefaults instantiates a new CSITopologyRequest object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewCSITopologyRequestWithDefaults() *CSITopologyRequest {
	this := CSITopologyRequest{}
	return &this
}

// GetPreferred returns the Preferred field value if set, zero value otherwise.
func (o *CSITopologyRequest) GetPreferred() []CSITopology {
	if o == nil || o.Preferred == nil {
		var ret []CSITopology
		return ret
	}
	return *o.Preferred
}

// GetPreferredOk returns a tuple with the Preferred field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *CSITopologyRequest) GetPreferredOk() (*[]CSITopology, bool) {
	if o == nil || o.Preferred == nil {
		return nil, false
	}
	return o.Preferred, true
}

// HasPreferred returns a boolean if a field has been set.
func (o *CSITopologyRequest) HasPreferred() bool {
	if o != nil && o.Preferred != nil {
		return true
	}

	return false
}

// SetPreferred gets a reference to the given []CSITopology and assigns it to the Preferred field.
func (o *CSITopologyRequest) SetPreferred(v []CSITopology) {
	o.Preferred = &v
}

// GetRequired returns the Required field value if set, zero value otherwise.
func (o *CSITopologyRequest) GetRequired() []CSITopology {
	if o == nil || o.Required == nil {
		var ret []CSITopology
		return ret
	}
	return *o.Required
}

// GetRequiredOk returns a tuple with the Required field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *CSITopologyRequest) GetRequiredOk() (*[]CSITopology, bool) {
	if o == nil || o.Required == nil {
		return nil, false
	}
	return o.Required, true
}

// HasRequired returns a boolean if a field has been set.
func (o *CSITopologyRequest) HasRequired() bool {
	if o != nil && o.Required != nil {
		return true
	}

	return false
}

// SetRequired gets a reference to the given []CSITopology and assigns it to the Required field.
func (o *CSITopologyRequest) SetRequired(v []CSITopology) {
	o.Required = &v
}

func (o CSITopologyRequest) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Preferred != nil {
		toSerialize["Preferred"] = o.Preferred
	}
	if o.Required != nil {
		toSerialize["Required"] = o.Required
	}
	return json.Marshal(toSerialize)
}

type NullableCSITopologyRequest struct {
	value *CSITopologyRequest
	isSet bool
}

func (v NullableCSITopologyRequest) Get() *CSITopologyRequest {
	return v.value
}

func (v *NullableCSITopologyRequest) Set(val *CSITopologyRequest) {
	v.value = val
	v.isSet = true
}

func (v NullableCSITopologyRequest) IsSet() bool {
	return v.isSet
}

func (v *NullableCSITopologyRequest) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableCSITopologyRequest(val *CSITopologyRequest) *NullableCSITopologyRequest {
	return &NullableCSITopologyRequest{value: val, isSet: true}
}

func (v NullableCSITopologyRequest) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableCSITopologyRequest) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


