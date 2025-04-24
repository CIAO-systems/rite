//
//  Generated code. Do not modify.
//  source: ciao/accounts/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use createRequestDescriptor instead')
const CreateRequest$json = {
  '1': 'CreateRequest',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
    {'1': 'password', '3': 2, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `CreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRequestDescriptor = $convert.base64Decode(
    'Cg1DcmVhdGVSZXF1ZXN0EjAKB2FjY291bnQYASABKAsyFi5jaWFvLmFjY291bnRzLkFjY291bn'
    'RSB2FjY291bnQSGgoIcGFzc3dvcmQYAiABKAlSCHBhc3N3b3Jk');

@$core.Deprecated('Use createResponseDescriptor instead')
const CreateResponse$json = {
  '1': 'CreateResponse',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
  ],
};

/// Descriptor for `CreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createResponseDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVSZXNwb25zZRIwCgdhY2NvdW50GAEgASgLMhYuY2lhby5hY2NvdW50cy5BY2NvdW'
    '50UgdhY2NvdW50');

@$core.Deprecated('Use getRequestDescriptor instead')
const GetRequest$json = {
  '1': 'GetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'id'},
    {'1': 'email', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'email'},
  ],
  '8': [
    {'1': 'identity'},
  ],
};

/// Descriptor for `GetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRequestDescriptor = $convert.base64Decode(
    'CgpHZXRSZXF1ZXN0EhAKAmlkGAEgASgJSABSAmlkEhYKBWVtYWlsGAIgASgJSABSBWVtYWlsQg'
    'oKCGlkZW50aXR5');

@$core.Deprecated('Use getResponseDescriptor instead')
const GetResponse$json = {
  '1': 'GetResponse',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRIwCgdhY2NvdW50GAEgASgLMhYuY2lhby5hY2NvdW50cy5BY2NvdW50Ug'
    'dhY2NvdW50');

@$core.Deprecated('Use listRequestDescriptor instead')
const ListRequest$json = {
  '1': 'ListRequest',
};

/// Descriptor for `ListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRequestDescriptor = $convert.base64Decode(
    'CgtMaXN0UmVxdWVzdA==');

@$core.Deprecated('Use listResponseDescriptor instead')
const ListResponse$json = {
  '1': 'ListResponse',
  '2': [
    {'1': 'accounts', '3': 1, '4': 3, '5': 11, '6': '.ciao.accounts.Account', '10': 'accounts'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USMgoIYWNjb3VudHMYASADKAsyFi5jaWFvLmFjY291bnRzLkFjY291bn'
    'RSCGFjY291bnRz');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'account', '3': 2, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0EjAKB2FjY291bnQYAiABKAsyFi5jaWFvLmFjY291bnRzLkFjY291bn'
    'RSB2FjY291bnQ=');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'account', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.Account', '10': 'account'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRIwCgdhY2NvdW50GAEgASgLMhYuY2lhby5hY2NvdW50cy5BY2NvdW'
    '50UgdhY2NvdW50');

@$core.Deprecated('Use deleteRequestDescriptor instead')
const DeleteRequest$json = {
  '1': 'DeleteRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `DeleteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteRequestDescriptor = $convert.base64Decode(
    'Cg1EZWxldGVSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZA==');

@$core.Deprecated('Use deleteResponseDescriptor instead')
const DeleteResponse$json = {
  '1': 'DeleteResponse',
};

/// Descriptor for `DeleteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteResponseDescriptor = $convert.base64Decode(
    'Cg5EZWxldGVSZXNwb25zZQ==');

@$core.Deprecated('Use personalMobileActionsRequestDescriptor instead')
const PersonalMobileActionsRequest$json = {
  '1': 'PersonalMobileActionsRequest',
};

/// Descriptor for `PersonalMobileActionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personalMobileActionsRequestDescriptor = $convert.base64Decode(
    'ChxQZXJzb25hbE1vYmlsZUFjdGlvbnNSZXF1ZXN0');

@$core.Deprecated('Use personalMobileActionsResponseDescriptor instead')
const PersonalMobileActionsResponse$json = {
  '1': 'PersonalMobileActionsResponse',
  '2': [
    {'1': 'actions', '3': 1, '4': 3, '5': 11, '6': '.ciao.devices.DeviceAction', '10': 'actions'},
  ],
};

/// Descriptor for `PersonalMobileActionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List personalMobileActionsResponseDescriptor = $convert.base64Decode(
    'Ch1QZXJzb25hbE1vYmlsZUFjdGlvbnNSZXNwb25zZRI0CgdhY3Rpb25zGAEgAygLMhouY2lhby'
    '5kZXZpY2VzLkRldmljZUFjdGlvblIHYWN0aW9ucw==');

@$core.Deprecated('Use createVersionRequestDescriptor instead')
const CreateVersionRequest$json = {
  '1': 'CreateVersionRequest',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'version'},
  ],
};

/// Descriptor for `CreateVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createVersionRequestDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVWZXJzaW9uUmVxdWVzdBI3Cgd2ZXJzaW9uGAEgASgLMh0uY2lhby5hY2NvdW50cy'
    '5BY2NvdW50VmVyc2lvblIHdmVyc2lvbg==');

@$core.Deprecated('Use createVersionResponseDescriptor instead')
const CreateVersionResponse$json = {
  '1': 'CreateVersionResponse',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'version'},
  ],
};

/// Descriptor for `CreateVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createVersionResponseDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVWZXJzaW9uUmVzcG9uc2USNwoHdmVyc2lvbhgBIAEoCzIdLmNpYW8uYWNjb3VudH'
    'MuQWNjb3VudFZlcnNpb25SB3ZlcnNpb24=');

@$core.Deprecated('Use getVersionRequestDescriptor instead')
const GetVersionRequest$json = {
  '1': 'GetVersionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `GetVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getVersionRequestDescriptor = $convert.base64Decode(
    'ChFHZXRWZXJzaW9uUmVxdWVzdBIOCgJpZBgBIAEoCVICaWQ=');

@$core.Deprecated('Use getVersionResponseDescriptor instead')
const GetVersionResponse$json = {
  '1': 'GetVersionResponse',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'version'},
  ],
};

/// Descriptor for `GetVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getVersionResponseDescriptor = $convert.base64Decode(
    'ChJHZXRWZXJzaW9uUmVzcG9uc2USNwoHdmVyc2lvbhgBIAEoCzIdLmNpYW8uYWNjb3VudHMuQW'
    'Njb3VudFZlcnNpb25SB3ZlcnNpb24=');

@$core.Deprecated('Use listVersionsRequestDescriptor instead')
const ListVersionsRequest$json = {
  '1': 'ListVersionsRequest',
  '2': [
    {'1': 'user_ids', '3': 1, '4': 3, '5': 9, '10': 'userIds'},
  ],
};

/// Descriptor for `ListVersionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVersionsRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0VmVyc2lvbnNSZXF1ZXN0EhkKCHVzZXJfaWRzGAEgAygJUgd1c2VySWRz');

@$core.Deprecated('Use listVersionsResponseDescriptor instead')
const ListVersionsResponse$json = {
  '1': 'ListVersionsResponse',
  '2': [
    {'1': 'versions', '3': 1, '4': 3, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'versions'},
  ],
};

/// Descriptor for `ListVersionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listVersionsResponseDescriptor = $convert.base64Decode(
    'ChRMaXN0VmVyc2lvbnNSZXNwb25zZRI5Cgh2ZXJzaW9ucxgBIAMoCzIdLmNpYW8uYWNjb3VudH'
    'MuQWNjb3VudFZlcnNpb25SCHZlcnNpb25z');

@$core.Deprecated('Use updateVersionRequestDescriptor instead')
const UpdateVersionRequest$json = {
  '1': 'UpdateVersionRequest',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'version'},
  ],
};

/// Descriptor for `UpdateVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVersionRequestDescriptor = $convert.base64Decode(
    'ChRVcGRhdGVWZXJzaW9uUmVxdWVzdBI3Cgd2ZXJzaW9uGAEgASgLMh0uY2lhby5hY2NvdW50cy'
    '5BY2NvdW50VmVyc2lvblIHdmVyc2lvbg==');

@$core.Deprecated('Use updateVersionResponseDescriptor instead')
const UpdateVersionResponse$json = {
  '1': 'UpdateVersionResponse',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 11, '6': '.ciao.accounts.AccountVersion', '10': 'version'},
  ],
};

/// Descriptor for `UpdateVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateVersionResponseDescriptor = $convert.base64Decode(
    'ChVVcGRhdGVWZXJzaW9uUmVzcG9uc2USNwoHdmVyc2lvbhgBIAEoCzIdLmNpYW8uYWNjb3VudH'
    'MuQWNjb3VudFZlcnNpb25SB3ZlcnNpb24=');

@$core.Deprecated('Use deleteVersionRequestDescriptor instead')
const DeleteVersionRequest$json = {
  '1': 'DeleteVersionRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `DeleteVersionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteVersionRequestDescriptor = $convert.base64Decode(
    'ChREZWxldGVWZXJzaW9uUmVxdWVzdBIOCgJpZBgBIAEoCVICaWQ=');

@$core.Deprecated('Use deleteVersionResponseDescriptor instead')
const DeleteVersionResponse$json = {
  '1': 'DeleteVersionResponse',
};

/// Descriptor for `DeleteVersionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteVersionResponseDescriptor = $convert.base64Decode(
    'ChVEZWxldGVWZXJzaW9uUmVzcG9uc2U=');

