//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/absences/service.proto
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
    {'1': 'absence', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absence'},
  ],
};

/// Descriptor for `CreateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createRequestDescriptor = $convert.base64Decode(
    'Cg1DcmVhdGVSZXF1ZXN0Ej4KB2Fic2VuY2UYASABKAsyJC5jaWFvLnRpbWVfdHJhY2tpbmcuYW'
    'JzZW5jZXMuQWJzZW5jZVIHYWJzZW5jZQ==');

@$core.Deprecated('Use createResponseDescriptor instead')
const CreateResponse$json = {
  '1': 'CreateResponse',
  '2': [
    {'1': 'absence', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absence'},
  ],
};

/// Descriptor for `CreateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createResponseDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVSZXNwb25zZRI+CgdhYnNlbmNlGAEgASgLMiQuY2lhby50aW1lX3RyYWNraW5nLm'
    'Fic2VuY2VzLkFic2VuY2VSB2Fic2VuY2U=');

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
    {'1': 'absence', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absence'},
  ],
};

/// Descriptor for `GetResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getResponseDescriptor = $convert.base64Decode(
    'CgtHZXRSZXNwb25zZRI+CgdhYnNlbmNlGAEgASgLMiQuY2lhby50aW1lX3RyYWNraW5nLmFic2'
    'VuY2VzLkFic2VuY2VSB2Fic2VuY2U=');

@$core.Deprecated('Use updateRequestDescriptor instead')
const UpdateRequest$json = {
  '1': 'UpdateRequest',
  '2': [
    {'1': 'absence', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absence'},
  ],
};

/// Descriptor for `UpdateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateRequestDescriptor = $convert.base64Decode(
    'Cg1VcGRhdGVSZXF1ZXN0Ej4KB2Fic2VuY2UYASABKAsyJC5jaWFvLnRpbWVfdHJhY2tpbmcuYW'
    'JzZW5jZXMuQWJzZW5jZVIHYWJzZW5jZQ==');

@$core.Deprecated('Use updateResponseDescriptor instead')
const UpdateResponse$json = {
  '1': 'UpdateResponse',
  '2': [
    {'1': 'absence', '3': 1, '4': 1, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absence'},
  ],
};

/// Descriptor for `UpdateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateResponseDescriptor = $convert.base64Decode(
    'Cg5VcGRhdGVSZXNwb25zZRI+CgdhYnNlbmNlGAEgASgLMiQuY2lhby50aW1lX3RyYWNraW5nLm'
    'Fic2VuY2VzLkFic2VuY2VSB2Fic2VuY2U=');

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

@$core.Deprecated('Use listRequestDescriptor instead')
const ListRequest$json = {
  '1': 'ListRequest',
  '2': [
    {'1': 'time_range', '3': 1, '4': 1, '5': 11, '6': '.ciao.common.TimeRange', '9': 0, '10': 'timeRange', '17': true},
    {'1': 'user_ids', '3': 2, '4': 3, '5': 9, '10': 'userIds'},
    {'1': 'time_type_ids', '3': 3, '4': 3, '5': 9, '10': 'timeTypeIds'},
  ],
  '8': [
    {'1': '_time_range'},
  ],
};

/// Descriptor for `ListRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRequestDescriptor = $convert.base64Decode(
    'CgtMaXN0UmVxdWVzdBI6Cgp0aW1lX3JhbmdlGAEgASgLMhYuY2lhby5jb21tb24uVGltZVJhbm'
    'dlSABSCXRpbWVSYW5nZYgBARIZCgh1c2VyX2lkcxgCIAMoCVIHdXNlcklkcxIiCg10aW1lX3R5'
    'cGVfaWRzGAMgAygJUgt0aW1lVHlwZUlkc0INCgtfdGltZV9yYW5nZQ==');

@$core.Deprecated('Use listResponseDescriptor instead')
const ListResponse$json = {
  '1': 'ListResponse',
  '2': [
    {'1': 'absences', '3': 1, '4': 3, '5': 11, '6': '.ciao.time_tracking.absences.Absence', '10': 'absences'},
  ],
};

/// Descriptor for `ListResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listResponseDescriptor = $convert.base64Decode(
    'CgxMaXN0UmVzcG9uc2USQAoIYWJzZW5jZXMYASADKAsyJC5jaWFvLnRpbWVfdHJhY2tpbmcuYW'
    'JzZW5jZXMuQWJzZW5jZVIIYWJzZW5jZXM=');

