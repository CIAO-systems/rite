//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/absences/absence.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use absenceDescriptor instead')
const Absence$json = {
  '1': 'Absence',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'start_date', '3': 2, '4': 1, '5': 11, '6': '.ciao.common.Date', '10': 'startDate'},
    {'1': 'end_date', '3': 3, '4': 1, '5': 11, '6': '.ciao.common.Date', '10': 'endDate'},
    {'1': 'start_half_day', '3': 4, '4': 1, '5': 8, '10': 'startHalfDay'},
    {'1': 'end_half_day', '3': 5, '4': 1, '5': 8, '10': 'endHalfDay'},
    {'1': 'time_type_id', '3': 6, '4': 1, '5': 9, '10': 'timeTypeId'},
    {'1': 'user_id', '3': 7, '4': 1, '5': 9, '10': 'userId'},
    {'1': 'deleted', '3': 8, '4': 1, '5': 8, '10': 'deleted'},
  ],
};

/// Descriptor for `Absence`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List absenceDescriptor = $convert.base64Decode(
    'CgdBYnNlbmNlEg4KAmlkGAEgASgJUgJpZBIwCgpzdGFydF9kYXRlGAIgASgLMhEuY2lhby5jb2'
    '1tb24uRGF0ZVIJc3RhcnREYXRlEiwKCGVuZF9kYXRlGAMgASgLMhEuY2lhby5jb21tb24uRGF0'
    'ZVIHZW5kRGF0ZRIkCg5zdGFydF9oYWxmX2RheRgEIAEoCFIMc3RhcnRIYWxmRGF5EiAKDGVuZF'
    '9oYWxmX2RheRgFIAEoCFIKZW5kSGFsZkRheRIgCgx0aW1lX3R5cGVfaWQYBiABKAlSCnRpbWVU'
    'eXBlSWQSFwoHdXNlcl9pZBgHIAEoCVIGdXNlcklkEhgKB2RlbGV0ZWQYCCABKAhSB2RlbGV0ZW'
    'Q=');

