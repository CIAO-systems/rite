//
//  Generated code. Do not modify.
//  source: ciao/devices/device.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class DeviceType extends $pb.ProtobufEnum {
  static const DeviceType UNKNOWN = DeviceType._(0, _omitEnumNames ? '' : 'UNKNOWN');
  static const DeviceType DATAFOX = DeviceType._(1, _omitEnumNames ? '' : 'DATAFOX');
  static const DeviceType MOBILE = DeviceType._(2, _omitEnumNames ? '' : 'MOBILE');

  static const $core.List<DeviceType> values = <DeviceType> [
    UNKNOWN,
    DATAFOX,
    MOBILE,
  ];

  static final $core.Map<$core.int, DeviceType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DeviceType? valueOf($core.int value) => _byValue[value];

  const DeviceType._($core.int v, $core.String n) : super(v, n);
}

class DeviceActionType extends $pb.ProtobufEnum {
  static const DeviceActionType CLOCK = DeviceActionType._(0, _omitEnumNames ? '' : 'CLOCK');
  static const DeviceActionType CUSTOM = DeviceActionType._(1, _omitEnumNames ? '' : 'CUSTOM');

  static const $core.List<DeviceActionType> values = <DeviceActionType> [
    CLOCK,
    CUSTOM,
  ];

  static final $core.Map<$core.int, DeviceActionType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DeviceActionType? valueOf($core.int value) => _byValue[value];

  const DeviceActionType._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
