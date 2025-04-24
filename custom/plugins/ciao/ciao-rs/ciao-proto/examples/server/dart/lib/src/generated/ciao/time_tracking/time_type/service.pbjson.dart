//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/service.proto
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
    {'1': 'time_type', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeType'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRJDCgl0aW1lX3R5cGUYASABKAsyJi5jaWFvLnRpbWVfdHJhY2tpbmcudG'
    'ltZV90eXBlLlRpbWVUeXBlUgh0aW1lVHlwZQ==');

@$core.Deprecated('Use listRequestDescriptor instead')
const ListRequest$json = {
  '1': 'ListRequest',
  '2': [
    {'1': 'absence', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'absence', '17': true},
    {'1': 'bookable', '3': 2, '4': 1, '5': 8, '9': 1, '10': 'bookable', '17': true},
  ],
  '8': [
    {'1': '_absence'},
    {'1': '_bookable'},
  ],
};

/// Descriptor for `ListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRequestDescriptor = $convert.base64Decode(
    'CgtMaXN0UmVxdWVzdBIdCgdhYnNlbmNlGAEgASgISABSB2Fic2VuY2WIAQESHwoIYm9va2FibG'
    'UYAiABKAhIAVIIYm9va2FibGWIAQFCCgoIX2Fic2VuY2VCCwoJX2Jvb2thYmxl');

@$core.Deprecated('Use listResponseDescriptor instead')
const ListResponse$json = {
  '1': 'ListResponse',
  '2': [
    {'1': 'time_types', '3': 1, '4': 3, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeTypes'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USRQoKdGltZV90eXBlcxgBIAMoCzImLmNpYW8udGltZV90cmFja2luZy'
    '50aW1lX3R5cGUuVGltZVR5cGVSCXRpbWVUeXBlcw==');

@$core.Deprecated('Use createRequestDescriptor instead')
const CreateRequest$json = {
  '1': 'CreateRequest',
  '2': [
    {'1': 'time_type', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeType'},
  ],
};

/// Descriptor for `CreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRequestDescriptor = $convert.base64Decode(
    'Cg1DcmVhdGVSZXF1ZXN0EkMKCXRpbWVfdHlwZRgBIAEoCzImLmNpYW8udGltZV90cmFja2luZy'
    '50aW1lX3R5cGUuVGltZVR5cGVSCHRpbWVUeXBl');

@$core.Deprecated('Use createResponseDescriptor instead')
const CreateResponse$json = {
  '1': 'CreateResponse',
  '2': [
    {'1': 'time_type', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeType'},
  ],
};

/// Descriptor for `CreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createResponseDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVSZXNwb25zZRJDCgl0aW1lX3R5cGUYASABKAsyJi5jaWFvLnRpbWVfdHJhY2tpbm'
    'cudGltZV90eXBlLlRpbWVUeXBlUgh0aW1lVHlwZQ==');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'time_type', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeType'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0EkMKCXRpbWVfdHlwZRgBIAEoCzImLmNpYW8udGltZV90cmFja2luZy'
    '50aW1lX3R5cGUuVGltZVR5cGVSCHRpbWVUeXBl');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'time_type', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.time_type.TimeType', '10': 'timeType'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRJDCgl0aW1lX3R5cGUYASABKAsyJi5jaWFvLnRpbWVfdHJhY2tpbm'
    'cudGltZV90eXBlLlRpbWVUeXBlUgh0aW1lVHlwZQ==');

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

