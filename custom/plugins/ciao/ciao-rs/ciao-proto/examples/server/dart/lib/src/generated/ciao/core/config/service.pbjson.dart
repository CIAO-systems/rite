//
//  Generated code. Do not modify.
//  source: ciao/core/config/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use configurationValuesRequestDescriptor instead')
const ConfigurationValuesRequest$json = {
  '1': 'ConfigurationValuesRequest',
  '2': [
    {'1': 'keys', '3': 1, '4': 3, '5': 9, '10': 'keys'},
  ],
};

/// Descriptor for `ConfigurationValuesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List configurationValuesRequestDescriptor = $convert.base64Decode(
    'ChpDb25maWd1cmF0aW9uVmFsdWVzUmVxdWVzdBISCgRrZXlzGAEgAygJUgRrZXlz');

@$core.Deprecated('Use configurationValuesResponseDescriptor instead')
const ConfigurationValuesResponse$json = {
  '1': 'ConfigurationValuesResponse',
  '2': [
    {'1': 'entries', '3': 1, '4': 3, '5': 11, '6': '.ciao.core.config.ConfigurationValuesResponse.EntriesEntry', '10': 'entries'},
  ],
  '3': [ConfigurationValuesResponse_EntriesEntry$json],
};

@$core.Deprecated('Use configurationValuesResponseDescriptor instead')
const ConfigurationValuesResponse_EntriesEntry$json = {
  '1': 'EntriesEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {'1': 'value', '3': 2, '4': 1, '5': 9, '10': 'value'},
  ],
  '7': {'7': true},
};

/// Descriptor for `ConfigurationValuesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List configurationValuesResponseDescriptor = $convert.base64Decode(
    'ChtDb25maWd1cmF0aW9uVmFsdWVzUmVzcG9uc2USVAoHZW50cmllcxgBIAMoCzI6LmNpYW8uY2'
    '9yZS5jb25maWcuQ29uZmlndXJhdGlvblZhbHVlc1Jlc3BvbnNlLkVudHJpZXNFbnRyeVIHZW50'
    'cmllcxo6CgxFbnRyaWVzRW50cnkSEAoDa2V5GAEgASgJUgNrZXkSFAoFdmFsdWUYAiABKAlSBX'
    'ZhbHVlOgI4AQ==');

@$core.Deprecated('Use serverInfoRequestDescriptor instead')
const ServerInfoRequest$json = {
  '1': 'ServerInfoRequest',
};

/// Descriptor for `ServerInfoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverInfoRequestDescriptor = $convert.base64Decode(
    'ChFTZXJ2ZXJJbmZvUmVxdWVzdA==');

@$core.Deprecated('Use serverInfoResponseDescriptor instead')
const ServerInfoResponse$json = {
  '1': 'ServerInfoResponse',
  '2': [
    {'1': 'version', '3': 1, '4': 1, '5': 9, '10': 'version'},
    {'1': 'build_time', '3': 2, '4': 1, '5': 9, '10': 'buildTime'},
  ],
};

/// Descriptor for `ServerInfoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List serverInfoResponseDescriptor = $convert.base64Decode(
    'ChJTZXJ2ZXJJbmZvUmVzcG9uc2USGAoHdmVyc2lvbhgBIAEoCVIHdmVyc2lvbhIdCgpidWlsZF'
    '90aW1lGAIgASgJUglidWlsZFRpbWU=');

