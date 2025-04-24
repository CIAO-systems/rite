//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use getRequestDescriptor instead')
const GetRequest$json = {
  '1': 'GetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `GetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRequestDescriptor = $convert.base64Decode(
    'CgpHZXRSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZA==');

@$core.Deprecated('Use getResponseDescriptor instead')
const GetResponse$json = {
  '1': 'GetResponse',
  '2': [
    {'1': 'record', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.ClockRecord', '10': 'record'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRI3CgZyZWNvcmQYASABKAsyHy5jaWFvLnRpbWVfdHJhY2tpbmcuQ2xvY2'
    'tSZWNvcmRSBnJlY29yZA==');

@$core.Deprecated('Use listRequestDescriptor instead')
const ListRequest$json = {
  '1': 'ListRequest',
  '2': [
    {'1': 'time_range', '3': 1, '4': 1, '5': 11, '6': '.ciao.common.TimeRange', '9': 0, '10': 'timeRange', '17': true},
    {'1': 'user_id', '3': 2, '4': 1, '5': 9, '9': 1, '10': 'userId', '17': true},
    {'1': 'creator_id', '3': 3, '4': 1, '5': 9, '9': 2, '10': 'creatorId', '17': true},
    {'1': 'time_type_id', '3': 4, '4': 1, '5': 9, '9': 3, '10': 'timeTypeId', '17': true},
  ],
  '8': [
    {'1': '_time_range'},
    {'1': '_user_id'},
    {'1': '_creator_id'},
    {'1': '_time_type_id'},
  ],
};

/// Descriptor for `ListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRequestDescriptor = $convert.base64Decode(
    'CgtMaXN0UmVxdWVzdBI6Cgp0aW1lX3JhbmdlGAEgASgLMhYuY2lhby5jb21tb24uVGltZVJhbm'
    'dlSABSCXRpbWVSYW5nZYgBARIcCgd1c2VyX2lkGAIgASgJSAFSBnVzZXJJZIgBARIiCgpjcmVh'
    'dG9yX2lkGAMgASgJSAJSCWNyZWF0b3JJZIgBARIlCgx0aW1lX3R5cGVfaWQYBCABKAlIA1IKdG'
    'ltZVR5cGVJZIgBAUINCgtfdGltZV9yYW5nZUIKCghfdXNlcl9pZEINCgtfY3JlYXRvcl9pZEIP'
    'Cg1fdGltZV90eXBlX2lk');

@$core.Deprecated('Use listResponseDescriptor instead')
const ListResponse$json = {
  '1': 'ListResponse',
  '2': [
    {'1': 'records', '3': 1, '4': 3, '5': 11, '6': '.ciao.time_tracking.ClockRecord', '10': 'records'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USOQoHcmVjb3JkcxgBIAMoCzIfLmNpYW8udGltZV90cmFja2luZy5DbG'
    '9ja1JlY29yZFIHcmVjb3Jkcw==');

@$core.Deprecated('Use clockResponseDescriptor instead')
const ClockResponse$json = {
  '1': 'ClockResponse',
  '2': [
    {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
    {'1': 'record', '3': 2, '4': 1, '5': 11, '6': '.ciao.time_tracking.ClockRecord', '10': 'record'},
  ],
};

/// Descriptor for `ClockResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List clockResponseDescriptor = $convert.base64Decode(
    'Cg1DbG9ja1Jlc3BvbnNlEhgKB21lc3NhZ2UYASABKAlSB21lc3NhZ2USNwoGcmVjb3JkGAIgAS'
    'gLMh8uY2lhby50aW1lX3RyYWNraW5nLkNsb2NrUmVjb3JkUgZyZWNvcmQ=');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'record', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.ClockRecord', '10': 'record'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0EjcKBnJlY29yZBgBIAEoCzIfLmNpYW8udGltZV90cmFja2luZy5DbG'
    '9ja1JlY29yZFIGcmVjb3Jk');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'record', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.ClockRecord', '10': 'record'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRI3CgZyZWNvcmQYASABKAsyHy5jaWFvLnRpbWVfdHJhY2tpbmcuQ2'
    'xvY2tSZWNvcmRSBnJlY29yZA==');

@$core.Deprecated('Use userInfoRequestDescriptor instead')
const UserInfoRequest$json = {
  '1': 'UserInfoRequest',
  '2': [
    {'1': 'user_id', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'userId'},
    {'1': 'badge_id', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'badgeId'},
  ],
  '8': [
    {'1': 'identity'},
  ],
};

/// Descriptor for `UserInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoRequestDescriptor = $convert.base64Decode(
    'Cg9Vc2VySW5mb1JlcXVlc3QSGQoHdXNlcl9pZBgBIAEoCUgAUgZ1c2VySWQSGwoIYmFkZ2VfaW'
    'QYAiABKAlIAFIHYmFkZ2VJZEIKCghpZGVudGl0eQ==');

@$core.Deprecated('Use balanceInfoDescriptor instead')
const BalanceInfo$json = {
  '1': 'BalanceInfo',
  '2': [
    {'1': 'current', '3': 2, '4': 1, '5': 11, '6': '.google.protobuf.Duration', '10': 'current'},
  ],
};

/// Descriptor for `BalanceInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List balanceInfoDescriptor = $convert.base64Decode(
    'CgtCYWxhbmNlSW5mbxIzCgdjdXJyZW50GAIgASgLMhkuZ29vZ2xlLnByb3RvYnVmLkR1cmF0aW'
    '9uUgdjdXJyZW50');

@$core.Deprecated('Use vacationInfoDescriptor instead')
const VacationInfo$json = {
  '1': 'VacationInfo',
  '2': [
    {'1': 'entitlement', '3': 1, '4': 1, '5': 11, '6': '.google.protobuf.Duration', '9': 0, '10': 'entitlement', '17': true},
    {'1': 'remaining', '3': 2, '4': 1, '5': 11, '6': '.google.protobuf.Duration', '9': 1, '10': 'remaining', '17': true},
    {'1': 'planned', '3': 3, '4': 1, '5': 11, '6': '.google.protobuf.Duration', '9': 2, '10': 'planned', '17': true},
  ],
  '8': [
    {'1': '_entitlement'},
    {'1': '_remaining'},
    {'1': '_planned'},
  ],
};

/// Descriptor for `VacationInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List vacationInfoDescriptor = $convert.base64Decode(
    'CgxWYWNhdGlvbkluZm8SQAoLZW50aXRsZW1lbnQYASABKAsyGS5nb29nbGUucHJvdG9idWYuRH'
    'VyYXRpb25IAFILZW50aXRsZW1lbnSIAQESPAoJcmVtYWluaW5nGAIgASgLMhkuZ29vZ2xlLnBy'
    'b3RvYnVmLkR1cmF0aW9uSAFSCXJlbWFpbmluZ4gBARI4CgdwbGFubmVkGAMgASgLMhkuZ29vZ2'
    'xlLnByb3RvYnVmLkR1cmF0aW9uSAJSB3BsYW5uZWSIAQFCDgoMX2VudGl0bGVtZW50QgwKCl9y'
    'ZW1haW5pbmdCCgoIX3BsYW5uZWQ=');

@$core.Deprecated('Use userInfoResponseDescriptor instead')
const UserInfoResponse$json = {
  '1': 'UserInfoResponse',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
    {'1': 'balance', '3': 2, '4': 1, '5': 11, '6': '.ciao.time_tracking.BalanceInfo', '9': 0, '10': 'balance', '17': true},
    {'1': 'vacation', '3': 3, '4': 1, '5': 11, '6': '.ciao.time_tracking.VacationInfo', '9': 1, '10': 'vacation', '17': true},
  ],
  '8': [
    {'1': '_balance'},
    {'1': '_vacation'},
  ],
};

/// Descriptor for `UserInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userInfoResponseDescriptor = $convert.base64Decode(
    'ChBVc2VySW5mb1Jlc3BvbnNlEjAKB2FjY291bnQYASABKAsyFi5jaWFvLmFjY291bnRzLkFjY2'
    '91bnRSB2FjY291bnQSPgoHYmFsYW5jZRgCIAEoCzIfLmNpYW8udGltZV90cmFja2luZy5CYWxh'
    'bmNlSW5mb0gAUgdiYWxhbmNliAEBEkEKCHZhY2F0aW9uGAMgASgLMiAuY2lhby50aW1lX3RyYW'
    'NraW5nLlZhY2F0aW9uSW5mb0gBUgh2YWNhdGlvbogBAUIKCghfYmFsYW5jZUILCglfdmFjYXRp'
    'b24=');

