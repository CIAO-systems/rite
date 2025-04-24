//
//  Generated code. Do not modify.
//  source: ciao/core/auth/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use loginRequestDescriptor instead')
const LoginRequest$json = {
  '1': 'LoginRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'userId'},
    {'1': 'email', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'email'},
    {'1': 'secret', '3': 3, '4': 1, '5': 9, '10': 'secret'},
  ],
  '8': [
    {'1': 'identity'},
  ],
};

/// Descriptor for `LoginRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginRequestDescriptor = $convert.base64Decode(
    'CgxMb2dpblJlcXVlc3QSGQoHdXNlcl9pZBgBIAEoCUgAUgZ1c2VySWQSFgoFZW1haWwYAiABKA'
    'lIAFIFZW1haWwSFgoGc2VjcmV0GAMgASgJUgZzZWNyZXRCCgoIaWRlbnRpdHk=');

@$core.Deprecated('Use loginResultDescriptor instead')
const LoginResult$json = {
  '1': 'LoginResult',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
    {'1': 'token', '3': 2, '4': 1, '5': 9, '10': 'token'},
  ],
};

/// Descriptor for `LoginResult`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List loginResultDescriptor = $convert.base64Decode(
    'CgtMb2dpblJlc3VsdBIwCgdhY2NvdW50GAEgASgLMhYuY2lhby5hY2NvdW50cy5BY2NvdW50Ug'
    'dhY2NvdW50EhQKBXRva2VuGAIgASgJUgV0b2tlbg==');

