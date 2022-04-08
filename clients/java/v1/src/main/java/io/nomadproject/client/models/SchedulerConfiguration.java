/*
 * Nomad
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package io.nomadproject.client.models;

import java.util.Objects;
import java.util.Arrays;
import com.google.gson.TypeAdapter;
import com.google.gson.annotations.JsonAdapter;
import com.google.gson.annotations.SerializedName;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonWriter;
import io.nomadproject.client.models.PreemptionConfig;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.io.IOException;

/**
 * SchedulerConfiguration
 */
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen")
public class SchedulerConfiguration {
  public static final String SERIALIZED_NAME_CREATE_INDEX = "CreateIndex";
  @SerializedName(SERIALIZED_NAME_CREATE_INDEX)
  private Integer createIndex;

  public static final String SERIALIZED_NAME_MEMORY_OVERSUBSCRIPTION_ENABLED = "MemoryOversubscriptionEnabled";
  @SerializedName(SERIALIZED_NAME_MEMORY_OVERSUBSCRIPTION_ENABLED)
  private Boolean memoryOversubscriptionEnabled;

  public static final String SERIALIZED_NAME_MODIFY_INDEX = "ModifyIndex";
  @SerializedName(SERIALIZED_NAME_MODIFY_INDEX)
  private Integer modifyIndex;

  public static final String SERIALIZED_NAME_PREEMPTION_CONFIG = "PreemptionConfig";
  @SerializedName(SERIALIZED_NAME_PREEMPTION_CONFIG)
  private PreemptionConfig preemptionConfig;

  public static final String SERIALIZED_NAME_REJECT_JOB_REGISTRATION = "RejectJobRegistration";
  @SerializedName(SERIALIZED_NAME_REJECT_JOB_REGISTRATION)
  private Boolean rejectJobRegistration;

  public static final String SERIALIZED_NAME_SCHEDULER_ALGORITHM = "SchedulerAlgorithm";
  @SerializedName(SERIALIZED_NAME_SCHEDULER_ALGORITHM)
  private String schedulerAlgorithm;


  public SchedulerConfiguration createIndex(Integer createIndex) {
    
    this.createIndex = createIndex;
    return this;
  }

   /**
   * Get createIndex
   * minimum: 0
   * maximum: 384
   * @return createIndex
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public Integer getCreateIndex() {
    return createIndex;
  }


  public void setCreateIndex(Integer createIndex) {
    this.createIndex = createIndex;
  }


  public SchedulerConfiguration memoryOversubscriptionEnabled(Boolean memoryOversubscriptionEnabled) {
    
    this.memoryOversubscriptionEnabled = memoryOversubscriptionEnabled;
    return this;
  }

   /**
   * Get memoryOversubscriptionEnabled
   * @return memoryOversubscriptionEnabled
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public Boolean getMemoryOversubscriptionEnabled() {
    return memoryOversubscriptionEnabled;
  }


  public void setMemoryOversubscriptionEnabled(Boolean memoryOversubscriptionEnabled) {
    this.memoryOversubscriptionEnabled = memoryOversubscriptionEnabled;
  }


  public SchedulerConfiguration modifyIndex(Integer modifyIndex) {
    
    this.modifyIndex = modifyIndex;
    return this;
  }

   /**
   * Get modifyIndex
   * minimum: 0
   * maximum: 384
   * @return modifyIndex
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public Integer getModifyIndex() {
    return modifyIndex;
  }


  public void setModifyIndex(Integer modifyIndex) {
    this.modifyIndex = modifyIndex;
  }


  public SchedulerConfiguration preemptionConfig(PreemptionConfig preemptionConfig) {
    
    this.preemptionConfig = preemptionConfig;
    return this;
  }

   /**
   * Get preemptionConfig
   * @return preemptionConfig
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public PreemptionConfig getPreemptionConfig() {
    return preemptionConfig;
  }


  public void setPreemptionConfig(PreemptionConfig preemptionConfig) {
    this.preemptionConfig = preemptionConfig;
  }


  public SchedulerConfiguration rejectJobRegistration(Boolean rejectJobRegistration) {
    
    this.rejectJobRegistration = rejectJobRegistration;
    return this;
  }

   /**
   * Get rejectJobRegistration
   * @return rejectJobRegistration
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public Boolean getRejectJobRegistration() {
    return rejectJobRegistration;
  }


  public void setRejectJobRegistration(Boolean rejectJobRegistration) {
    this.rejectJobRegistration = rejectJobRegistration;
  }


  public SchedulerConfiguration schedulerAlgorithm(String schedulerAlgorithm) {
    
    this.schedulerAlgorithm = schedulerAlgorithm;
    return this;
  }

   /**
   * Get schedulerAlgorithm
   * @return schedulerAlgorithm
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public String getSchedulerAlgorithm() {
    return schedulerAlgorithm;
  }


  public void setSchedulerAlgorithm(String schedulerAlgorithm) {
    this.schedulerAlgorithm = schedulerAlgorithm;
  }


  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SchedulerConfiguration schedulerConfiguration = (SchedulerConfiguration) o;
    return Objects.equals(this.createIndex, schedulerConfiguration.createIndex) &&
        Objects.equals(this.memoryOversubscriptionEnabled, schedulerConfiguration.memoryOversubscriptionEnabled) &&
        Objects.equals(this.modifyIndex, schedulerConfiguration.modifyIndex) &&
        Objects.equals(this.preemptionConfig, schedulerConfiguration.preemptionConfig) &&
        Objects.equals(this.rejectJobRegistration, schedulerConfiguration.rejectJobRegistration) &&
        Objects.equals(this.schedulerAlgorithm, schedulerConfiguration.schedulerAlgorithm);
  }

  @Override
  public int hashCode() {
    return Objects.hash(createIndex, memoryOversubscriptionEnabled, modifyIndex, preemptionConfig, rejectJobRegistration, schedulerAlgorithm);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SchedulerConfiguration {\n");
    sb.append("    createIndex: ").append(toIndentedString(createIndex)).append("\n");
    sb.append("    memoryOversubscriptionEnabled: ").append(toIndentedString(memoryOversubscriptionEnabled)).append("\n");
    sb.append("    modifyIndex: ").append(toIndentedString(modifyIndex)).append("\n");
    sb.append("    preemptionConfig: ").append(toIndentedString(preemptionConfig)).append("\n");
    sb.append("    rejectJobRegistration: ").append(toIndentedString(rejectJobRegistration)).append("\n");
    sb.append("    schedulerAlgorithm: ").append(toIndentedString(schedulerAlgorithm)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

}

