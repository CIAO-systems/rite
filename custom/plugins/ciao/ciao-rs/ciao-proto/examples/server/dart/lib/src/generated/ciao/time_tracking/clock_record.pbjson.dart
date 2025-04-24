//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/clock_record.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use clockRecordDescriptor instead')
const ClockRecord$json = {
  '1': 'ClockRecord',
  '2': [
    {'1': 'timestamp', '3': 1, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '10': 'timestamp'},
    {'1': 'user_id', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'userId'},
    {'1': 'badge_id', '3': 3, '4': 1, '5': 9, '9': 0, '10': 'badgeId'},
    {'1': 'device_id', '3': 4, '4': 1, '5': 9, '9': 1, '10': 'deviceId', '17': true},
    {'1': 'time_type_id', '3': 5, '4': 1, '5': 9, '9': 2, '10': 'timeTypeId', '17': true},
    {'1': 'project_id', '3': 100, '4': 1, '5': 9, '9': 3, '10': 'projectId', '17': true},
    {'1': 'cost_center_id', '3': 101, '4': 1, '5': 9, '9': 4, '10': 'costCenterId', '17': true},
    {'1': 'project_task_id', '3': 102, '4': 1, '5': 9, '9': 5, '10': 'projectTaskId', '17': true},
    {'1': 'id', '3': 200, '4': 1, '5': 9, '9': 6, '10': 'id', '17': true},
  ],
  '8': [
    {'1': 'identity'},
    {'1': '_device_id'},
    {'1': '_time_type_id'},
    {'1': '_project_id'},
    {'1': '_cost_center_id'},
    {'1': '_project_task_id'},
    {'1': '_id'},
  ],
};

/// Descriptor for `ClockRecord`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clockRecordDescriptor = $convert.base64Decode(
    'CgtDbG9ja1JlY29yZBI0Cgl0aW1lc3RhbXAYASABKAsyFi5jaWFvLmNvbW1vbi5UaW1lc3RhbX'
    'BSCXRpbWVzdGFtcBIZCgd1c2VyX2lkGAIgASgJSABSBnVzZXJJZBIbCghiYWRnZV9pZBgDIAEo'
    'CUgAUgdiYWRnZUlkEiAKCWRldmljZV9pZBgEIAEoCUgBUghkZXZpY2VJZIgBARIlCgx0aW1lX3'
    'R5cGVfaWQYBSABKAlIAlIKdGltZVR5cGVJZIgBARIiCgpwcm9qZWN0X2lkGGQgASgJSANSCXBy'
    'b2plY3RJZIgBARIpCg5jb3N0X2NlbnRlcl9pZBhlIAEoCUgEUgxjb3N0Q2VudGVySWSIAQESKw'
    'oPcHJvamVjdF90YXNrX2lkGGYgASgJSAVSDXByb2plY3RUYXNrSWSIAQESFAoCaWQYyAEgASgJ'
    'SAZSAmlkiAEBQgoKCGlkZW50aXR5QgwKCl9kZXZpY2VfaWRCDwoNX3RpbWVfdHlwZV9pZEINCg'
    'tfcHJvamVjdF9pZEIRCg9fY29zdF9jZW50ZXJfaWRCEgoQX3Byb2plY3RfdGFza19pZEIFCgNf'
    'aWQ=');

