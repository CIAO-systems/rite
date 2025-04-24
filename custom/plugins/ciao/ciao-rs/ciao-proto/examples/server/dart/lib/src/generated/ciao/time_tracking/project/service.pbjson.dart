//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/project/service.proto
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
    {'1': 'project', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'project'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRI9Cgdwcm9qZWN0GAEgASgLMiMuY2lhby50aW1lX3RyYWNraW5nLnByb2'
    'plY3QuUHJvamVjdFIHcHJvamVjdA==');

@$core.Deprecated('Use listRequestDescriptor instead')
const ListRequest$json = {
  '1': 'ListRequest',
  '2': [
    {'1': 'active_at', '3': 1, '4': 1, '5': 11, '6': '.ciao.common.Timestamp', '9': 0, '10': 'activeAt', '17': true},
  ],
  '8': [
    {'1': '_active_at'},
  ],
};

/// Descriptor for `ListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRequestDescriptor = $convert.base64Decode(
    'CgtMaXN0UmVxdWVzdBI4CglhY3RpdmVfYXQYASABKAsyFi5jaWFvLmNvbW1vbi5UaW1lc3RhbX'
    'BIAFIIYWN0aXZlQXSIAQFCDAoKX2FjdGl2ZV9hdA==');

@$core.Deprecated('Use listResponseDescriptor instead')
const ListResponse$json = {
  '1': 'ListResponse',
  '2': [
    {'1': 'projects', '3': 1, '4': 3, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'projects'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USPwoIcHJvamVjdHMYASADKAsyIy5jaWFvLnRpbWVfdHJhY2tpbmcucH'
    'JvamVjdC5Qcm9qZWN0Ughwcm9qZWN0cw==');

@$core.Deprecated('Use createRequestDescriptor instead')
const CreateRequest$json = {
  '1': 'CreateRequest',
  '2': [
    {'1': 'project', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'project'},
  ],
};

/// Descriptor for `CreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRequestDescriptor = $convert.base64Decode(
    'Cg1DcmVhdGVSZXF1ZXN0Ej0KB3Byb2plY3QYASABKAsyIy5jaWFvLnRpbWVfdHJhY2tpbmcucH'
    'JvamVjdC5Qcm9qZWN0Ugdwcm9qZWN0');

@$core.Deprecated('Use createResponseDescriptor instead')
const CreateResponse$json = {
  '1': 'CreateResponse',
  '2': [
    {'1': 'project', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'project'},
  ],
};

/// Descriptor for `CreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createResponseDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVSZXNwb25zZRI9Cgdwcm9qZWN0GAEgASgLMiMuY2lhby50aW1lX3RyYWNraW5nLn'
    'Byb2plY3QuUHJvamVjdFIHcHJvamVjdA==');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'project', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'project'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0Ej0KB3Byb2plY3QYASABKAsyIy5jaWFvLnRpbWVfdHJhY2tpbmcucH'
    'JvamVjdC5Qcm9qZWN0Ugdwcm9qZWN0');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'project', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.project.Project', '10': 'project'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRI9Cgdwcm9qZWN0GAEgASgLMiMuY2lhby50aW1lX3RyYWNraW5nLn'
    'Byb2plY3QuUHJvamVjdFIHcHJvamVjdA==');

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

