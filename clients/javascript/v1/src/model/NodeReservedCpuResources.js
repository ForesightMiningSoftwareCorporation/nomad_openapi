/**
 * Nomad
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */

import ApiClient from '../ApiClient';

/**
 * The NodeReservedCpuResources model module.
 * @module model/NodeReservedCpuResources
 * @version 1.1.4
 */
class NodeReservedCpuResources {
    /**
     * Constructs a new <code>NodeReservedCpuResources</code>.
     * @alias module:model/NodeReservedCpuResources
     */
    constructor() { 
        
        NodeReservedCpuResources.initialize(this);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj) { 
    }

    /**
     * Constructs a <code>NodeReservedCpuResources</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/NodeReservedCpuResources} obj Optional instance to populate.
     * @return {module:model/NodeReservedCpuResources} The populated <code>NodeReservedCpuResources</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new NodeReservedCpuResources();

            if (data.hasOwnProperty('CpuShares')) {
                obj['CpuShares'] = ApiClient.convertToType(data['CpuShares'], 'Number');
            }
        }
        return obj;
    }


}

/**
 * @member {Number} CpuShares
 */
NodeReservedCpuResources.prototype['CpuShares'] = undefined;






export default NodeReservedCpuResources;

