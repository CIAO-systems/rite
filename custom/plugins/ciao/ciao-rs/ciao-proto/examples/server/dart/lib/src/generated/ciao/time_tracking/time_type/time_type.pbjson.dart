//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/time_type.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use timeTypeDescriptor instead')
const TimeType$json = {
  '1': 'TimeType',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'shorthand', '3': 3, '4': 1, '5': 9, '10': 'shorthand'},
    {'1': 'color', '3': 4, '4': 1, '5': 11, '6': '.ciao.common.Color', '10': 'color'},
    {'1': 'icon', '3': 5, '4': 1, '5': 9, '9': 0, '10': 'icon', '17': true},
    {'1': 'options', '3': 6, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeTypeOptions', '10': 'options'},
  ],
  '8': [
    {'1': '_icon'},
  ],
};

/// Descriptor for `TimeType`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List timeTypeDescriptor = $convert.base64Decode(
    'CghUaW1lVHlwZRIOCgJpZBgBIAEoCVICaWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIcCglzaG9ydG'
    'hhbmQYAyABKAlSCXNob3J0aGFuZBIoCgVjb2xvchgEIAEoCzISLmNpYW8uY29tbW9uLkNvbG9y'
    'UgVjb2xvchIXCgRpY29uGAUgASgJSABSBGljb26IAQESRwoHb3B0aW9ucxgGIAEoCzItLmNpYW'
    '8udGltZV90cmFja2luZy50aW1lX3R5cGUuVGltZVR5cGVPcHRpb25zUgdvcHRpb25zQgcKBV9p'
    'Y29u');

@$core.Deprecated('Use timeTypeOptionsDescriptor instead')
const TimeTypeOptions$json = {
  '1': 'TimeTypeOptions',
  '2': [
    {'1': 'bookable', '3': 1, '4': 1, '5': 8, '10': 'bookable'},
    {'1': 'absence', '3': 2, '4': 1, '5': 8, '10': 'absence'},
  ],
};

/// Descriptor for `TimeTypeOptions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List timeTypeOptionsDescriptor = $convert.base64Decode(
    'Cg9UaW1lVHlwZU9wdGlvbnMSGgoIYm9va2FibGUYASABKAhSCGJvb2thYmxlEhgKB2Fic2VuY2'
    'UYAiABKAhSB2Fic2VuY2U=');

