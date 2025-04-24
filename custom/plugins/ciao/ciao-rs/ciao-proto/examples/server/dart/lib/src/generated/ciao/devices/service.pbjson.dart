//
//  Generated code. Do not modify.
//  source: ciao/devices/service.proto
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
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRIsCgZkZXZpY2UYASABKAsyFC5jaWFvLmRldmljZXMuRGV2aWNlUgZkZX'
    'ZpY2U=');

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
    {'1': 'devices', '3': 1, '4': 3, '5': 11, '6': '.ciao.devices.Device', '10': 'devices'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USLgoHZGV2aWNlcxgBIAMoCzIULmNpYW8uZGV2aWNlcy5EZXZpY2VSB2'
    'RldmljZXM=');

@$core.Deprecated('Use createRequestDescriptor instead')
const CreateRequest$json = {
  '1': 'CreateRequest',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `CreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRequestDescriptor = $convert.base64Decode(
    'Cg1DcmVhdGVSZXF1ZXN0EiwKBmRldmljZRgBIAEoCzIULmNpYW8uZGV2aWNlcy5EZXZpY2VSBm'
    'RldmljZQ==');

@$core.Deprecated('Use createResponseDescriptor instead')
const CreateResponse$json = {
  '1': 'CreateResponse',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `CreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createResponseDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVSZXNwb25zZRIsCgZkZXZpY2UYASABKAsyFC5jaWFvLmRldmljZXMuRGV2aWNlUg'
    'ZkZXZpY2U=');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0EiwKBmRldmljZRgBIAEoCzIULmNpYW8uZGV2aWNlcy5EZXZpY2VSBm'
    'RldmljZQ==');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRIsCgZkZXZpY2UYASABKAsyFC5jaWFvLmRldmljZXMuRGV2aWNlUg'
    'ZkZXZpY2U=');

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
  '2': [
    {'1': 'message', '3': 1, '4': 1, '5': 9, '10': 'message'},
  ],
};

/// Descriptor for `DeleteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deleteResponseDescriptor = $convert.base64Decode(
    'Cg5EZWxldGVSZXNwb25zZRIYCgdtZXNzYWdlGAEgASgJUgdtZXNzYWdl');

@$core.Deprecated('Use deviceConfigurationRequestDescriptor instead')
const DeviceConfigurationRequest$json = {
  '1': 'DeviceConfigurationRequest',
  '2': [
    {'1': 'external_id', '3': 1, '4': 1, '5': 9, '10': 'externalId'},
  ],
};

/// Descriptor for `DeviceConfigurationRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceConfigurationRequestDescriptor = $convert.base64Decode(
    'ChpEZXZpY2VDb25maWd1cmF0aW9uUmVxdWVzdBIfCgtleHRlcm5hbF9pZBgBIAEoCVIKZXh0ZX'
    'JuYWxJZA==');

@$core.Deprecated('Use deviceConfigurationResponseDescriptor instead')
const DeviceConfigurationResponse$json = {
  '1': 'DeviceConfigurationResponse',
  '2': [
    {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.ciao.devices.Device', '10': 'device'},
  ],
};

/// Descriptor for `DeviceConfigurationResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceConfigurationResponseDescriptor = $convert.base64Decode(
    'ChtEZXZpY2VDb25maWd1cmF0aW9uUmVzcG9uc2USLAoGZGV2aWNlGAEgASgLMhQuY2lhby5kZX'
    'ZpY2VzLkRldmljZVIGZGV2aWNl');

