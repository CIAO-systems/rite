//
//  Generated code. Do not modify.
//  source: ciao/common/address.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use addressDescriptor instead')
const Address$json = {
  '1': 'Address',
  '2': [
    {'1': 'city', '3': 1, '4': 1, '5': 9, '10': 'city'},
    {'1': 'postal_code', '3': 2, '4': 1, '5': 9, '10': 'postalCode'},
    {'1': 'address_line_1', '3': 3, '4': 1, '5': 9, '10': 'addressLine1'},
    {'1': 'address_line_2', '3': 6, '4': 1, '5': 9, '10': 'addressLine2'},
    {'1': 'region_code', '3': 4, '4': 1, '5': 9, '10': 'regionCode'},
    {'1': 'state', '3': 5, '4': 1, '5': 9, '10': 'state'},
  ],
};

/// Descriptor for `Address`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addressDescriptor = $convert.base64Decode(
    'CgdBZGRyZXNzEhIKBGNpdHkYASABKAlSBGNpdHkSHwoLcG9zdGFsX2NvZGUYAiABKAlSCnBvc3'
    'RhbENvZGUSJAoOYWRkcmVzc19saW5lXzEYAyABKAlSDGFkZHJlc3NMaW5lMRIkCg5hZGRyZXNz'
    'X2xpbmVfMhgGIAEoCVIMYWRkcmVzc0xpbmUyEh8KC3JlZ2lvbl9jb2RlGAQgASgJUgpyZWdpb2'
    '5Db2RlEhQKBXN0YXRlGAUgASgJUgVzdGF0ZQ==');

