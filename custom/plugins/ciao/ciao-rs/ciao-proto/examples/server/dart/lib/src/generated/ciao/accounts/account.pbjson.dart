//
//  Generated code. Do not modify.
//  source: ciao/accounts/account.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use accountDescriptor instead')
const Account$json = {
  '1': 'Account',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.ciao.common.Name', '10': 'name'},
    {'1': 'address', '3': 3, '4': 1, '5': 11, '6': '.ciao.common.Address', '10': 'address'},
    {'1': 'avatar', '3': 4, '4': 1, '5': 11, '6': '.ciao.common.Image', '10': 'avatar'},
    {'1': 'email', '3': 5, '4': 1, '5': 9, '10': 'email'},
  ],
};

/// Descriptor for `Account`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountDescriptor = $convert.base64Decode(
    'CgdBY2NvdW50Eg4KAmlkGAEgASgJUgJpZBIlCgRuYW1lGAIgASgLMhEuY2lhby5jb21tb24uTm'
    'FtZVIEbmFtZRIuCgdhZGRyZXNzGAMgASgLMhQuY2lhby5jb21tb24uQWRkcmVzc1IHYWRkcmVz'
    'cxIqCgZhdmF0YXIYBCABKAsyEi5jaWFvLmNvbW1vbi5JbWFnZVIGYXZhdGFyEhQKBWVtYWlsGA'
    'UgASgJUgVlbWFpbA==');

