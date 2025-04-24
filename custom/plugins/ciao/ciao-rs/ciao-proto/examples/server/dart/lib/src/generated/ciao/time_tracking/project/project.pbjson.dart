//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/project/project.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use projectDescriptor instead')
const Project$json = {
  '1': 'Project',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'external_id', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'externalId', '17': true},
    {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
    {'1': 'start_date', '3': 4, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '9': 1, '10': 'startDate', '17': true},
    {'1': 'end_date', '3': 5, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '9': 2, '10': 'endDate', '17': true},
    {'1': 'closed_date', '3': 6, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '9': 3, '10': 'closedDate', '17': true},
    {'1': 'parent_id', '3': 7, '4': 1, '5': 9, '9': 4, '10': 'parentId', '17': true},
  ],
  '8': [
    {'1': '_external_id'},
    {'1': '_start_date'},
    {'1': '_end_date'},
    {'1': '_closed_date'},
    {'1': '_parent_id'},
  ],
};

/// Descriptor for `Project`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List projectDescriptor = $convert.base64Decode(
    'CgdQcm9qZWN0Eg4KAmlkGAEgASgJUgJpZBIkCgtleHRlcm5hbF9pZBgCIAEoCUgAUgpleHRlcm'
    '5hbElkiAEBEhIKBG5hbWUYAyABKAlSBG5hbWUSOgoKc3RhcnRfZGF0ZRgEIAEoCzIWLmNpYW8u'
    'Y29tbW9uLlRpbWVzdGFtcEgBUglzdGFydERhdGWIAQESNgoIZW5kX2RhdGUYBSABKAsyFi5jaW'
    'FvLmNvbW1vbi5UaW1lc3RhbXBIAlIHZW5kRGF0ZYgBARI8CgtjbG9zZWRfZGF0ZRgGIAEoCzIW'
    'LmNpYW8uY29tbW9uLlRpbWVzdGFtcEgDUgpjbG9zZWREYXRliAEBEiAKCXBhcmVudF9pZBgHIA'
    'EoCUgEUghwYXJlbnRJZIgBAUIOCgxfZXh0ZXJuYWxfaWRCDQoLX3N0YXJ0X2RhdGVCCwoJX2Vu'
    'ZF9kYXRlQg4KDF9jbG9zZWRfZGF0ZUIMCgpfcGFyZW50X2lk');

