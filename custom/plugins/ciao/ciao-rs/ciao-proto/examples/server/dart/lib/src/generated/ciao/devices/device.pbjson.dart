//
//  Generated code. Do not modify.
//  source: ciao/devices/device.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use deviceTypeDescriptor instead')
const DeviceType$json = {
  '1': 'DeviceType',
  '2': [
    {'1': 'UNKNOWN', '2': 0},
    {'1': 'DATAFOX', '2': 1},
    {'1': 'MOBILE', '2': 2},
  ],
};

/// Descriptor for `DeviceType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List deviceTypeDescriptor = $convert.base64Decode(
    'CgpEZXZpY2VUeXBlEgsKB1VOS05PV04QABILCgdEQVRBRk9YEAESCgoGTU9CSUxFEAI=');

@$core.Deprecated('Use deviceActionTypeDescriptor instead')
const DeviceActionType$json = {
  '1': 'DeviceActionType',
  '2': [
    {'1': 'CLOCK', '2': 0},
    {'1': 'CUSTOM', '2': 1},
  ],
};

/// Descriptor for `DeviceActionType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List deviceActionTypeDescriptor = $convert.base64Decode(
    'ChBEZXZpY2VBY3Rpb25UeXBlEgkKBUNMT0NLEAASCgoGQ1VTVE9NEAE=');

@$core.Deprecated('Use deviceDescriptor instead')
const Device$json = {
  '1': 'Device',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'type', '3': 3, '4': 1, '5': 14, '6': '.ciao.devices.DeviceType', '10': 'type'},
    {'1': 'time_zone_id', '3': 4, '4': 1, '5': 9, '9': 0, '10': 'timeZoneId', '17': true},
    {'1': 'external_id', '3': 5, '4': 1, '5': 9, '10': 'externalId'},
    {'1': 'actions', '3': 10, '4': 3, '5': 11, '6': '.ciao.devices.DeviceAction', '10': 'actions'},
  ],
  '8': [
    {'1': '_time_zone_id'},
  ],
};

/// Descriptor for `Device`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceDescriptor = $convert.base64Decode(
    'CgZEZXZpY2USDgoCaWQYASABKAlSAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSLAoEdHlwZRgDIA'
    'EoDjIYLmNpYW8uZGV2aWNlcy5EZXZpY2VUeXBlUgR0eXBlEiUKDHRpbWVfem9uZV9pZBgEIAEo'
    'CUgAUgp0aW1lWm9uZUlkiAEBEh8KC2V4dGVybmFsX2lkGAUgASgJUgpleHRlcm5hbElkEjQKB2'
    'FjdGlvbnMYCiADKAsyGi5jaWFvLmRldmljZXMuRGV2aWNlQWN0aW9uUgdhY3Rpb25zQg8KDV90'
    'aW1lX3pvbmVfaWQ=');

@$core.Deprecated('Use deviceActionDescriptor instead')
const DeviceAction$json = {
  '1': 'DeviceAction',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 5, '10': 'id'},
    {'1': 'device_action_id', '3': 2, '4': 1, '5': 5, '10': 'deviceActionId'},
    {'1': 'icon', '3': 3, '4': 1, '5': 9, '9': 1, '10': 'icon', '17': true},
    {'1': 'name', '3': 4, '4': 1, '5': 9, '9': 2, '10': 'name', '17': true},
    {'1': 'type', '3': 5, '4': 1, '5': 14, '6': '.ciao.devices.DeviceActionType', '10': 'type'},
    {'1': 'clock', '3': 100, '4': 1, '5': 11, '6': '.ciao.devices.ClockConfiguration', '9': 0, '10': 'clock'},
    {'1': 'custom', '3': 101, '4': 1, '5': 11, '6': '.ciao.devices.CustomConfiguration', '9': 0, '10': 'custom'},
  ],
  '8': [
    {'1': 'configuration'},
    {'1': '_icon'},
    {'1': '_name'},
  ],
};

/// Descriptor for `DeviceAction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceActionDescriptor = $convert.base64Decode(
    'CgxEZXZpY2VBY3Rpb24SDgoCaWQYASABKAVSAmlkEigKEGRldmljZV9hY3Rpb25faWQYAiABKA'
    'VSDmRldmljZUFjdGlvbklkEhcKBGljb24YAyABKAlIAVIEaWNvbogBARIXCgRuYW1lGAQgASgJ'
    'SAJSBG5hbWWIAQESMgoEdHlwZRgFIAEoDjIeLmNpYW8uZGV2aWNlcy5EZXZpY2VBY3Rpb25UeX'
    'BlUgR0eXBlEjgKBWNsb2NrGGQgASgLMiAuY2lhby5kZXZpY2VzLkNsb2NrQ29uZmlndXJhdGlv'
    'bkgAUgVjbG9jaxI7CgZjdXN0b20YZSABKAsyIS5jaWFvLmRldmljZXMuQ3VzdG9tQ29uZmlndX'
    'JhdGlvbkgAUgZjdXN0b21CDwoNY29uZmlndXJhdGlvbkIHCgVfaWNvbkIHCgVfbmFtZQ==');

@$core.Deprecated('Use clockConfigurationDescriptor instead')
const ClockConfiguration$json = {
  '1': 'ClockConfiguration',
  '2': [
    {'1': 'time_type_id', '3': 1, '4': 1, '5': 9, '10': 'timeTypeId'},
    {'1': 'cost_center_id', '3': 2, '4': 1, '5': 9, '10': 'costCenterId'},
    {'1': 'project_id', '3': 3, '4': 1, '5': 9, '10': 'projectId'},
    {'1': 'project_task_id', '3': 4, '4': 1, '5': 9, '10': 'projectTaskId'},
  ],
};

/// Descriptor for `ClockConfiguration`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clockConfigurationDescriptor = $convert.base64Decode(
    'ChJDbG9ja0NvbmZpZ3VyYXRpb24SIAoMdGltZV90eXBlX2lkGAEgASgJUgp0aW1lVHlwZUlkEi'
    'QKDmNvc3RfY2VudGVyX2lkGAIgASgJUgxjb3N0Q2VudGVySWQSHQoKcHJvamVjdF9pZBgDIAEo'
    'CVIJcHJvamVjdElkEiYKD3Byb2plY3RfdGFza19pZBgEIAEoCVINcHJvamVjdFRhc2tJZA==');

@$core.Deprecated('Use customConfigurationDescriptor instead')
const CustomConfiguration$json = {
  '1': 'CustomConfiguration',
  '2': [
    {'1': 'operation', '3': 1, '4': 1, '5': 9, '10': 'operation'},
  ],
};

/// Descriptor for `CustomConfiguration`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List customConfigurationDescriptor = $convert.base64Decode(
    'ChNDdXN0b21Db25maWd1cmF0aW9uEhwKCW9wZXJhdGlvbhgBIAEoCVIJb3BlcmF0aW9u');

