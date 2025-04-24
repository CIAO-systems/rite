//
//  Generated code. Do not modify.
//  source: ciao/common/time_range.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use timeRangeDescriptor instead')
const TimeRange$json = {
  '1': 'TimeRange',
  '2': [
    {'1': 'start_time', '3': 1, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '10': 'startTime'},
    {'1': 'end_time', '3': 2, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '10': 'endTime'},
  ],
};

/// Descriptor for `TimeRange`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List timeRangeDescriptor = $convert.base64Decode(
    'CglUaW1lUmFuZ2USNQoKc3RhcnRfdGltZRgBIAEoCzIWLmNpYW8uY29tbW9uLlRpbWVzdGFtcF'
    'IJc3RhcnRUaW1lEjEKCGVuZF90aW1lGAIgASgLMhYuY2lhby5jb21tb24uVGltZXN0YW1wUgdl'
    'bmRUaW1l');

