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

import 'device.pbenum.dart';

export 'device.pbenum.dart';

class Device extends $pb.GeneratedMessage {
  factory Device({
    $core.String? id,
    $core.String? name,
    DeviceType? type,
    $core.String? timeZoneId,
    $core.String? externalId,
    $core.Iterable<DeviceAction>? actions,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (name != null) {
      result.name = name;
    }
    if (type != null) {
      result.type = type;
    }
    if (timeZoneId != null) {
      result.timeZoneId = timeZoneId;
    }
    if (externalId != null) {
      result.externalId = externalId;
    }
    if (actions != null) {
      result.actions.addAll(actions);
    }
    return result;
  }
  Device._() : super();
  factory Device.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Device.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Device', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.devices'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..e<DeviceType>(3, _omitFieldNames ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: DeviceType.UNKNOWN, valueOf: DeviceType.valueOf, enumValues: DeviceType.values)
    ..aOS(4, _omitFieldNames ? '' : 'timeZoneId')
    ..aOS(5, _omitFieldNames ? '' : 'externalId')
    ..pc<DeviceAction>(10, _omitFieldNames ? '' : 'actions', $pb.PbFieldType.PM, subBuilder: DeviceAction.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Device clone() => Device()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Device copyWith(void Function(Device) updates) => super.copyWith((message) => updates(message as Device)) as Device;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Device create() => Device._();
  Device createEmptyInstance() => create();
  static $pb.PbList<Device> createRepeated() => $pb.PbList<Device>();
  @$core.pragma('dart2js:noInline')
  static Device getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Device>(create);
  static Device? _defaultInstance;

  /// Unique id (UUID)
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  DeviceType get type => $_getN(2);
  @$pb.TagNumber(3)
  set type(DeviceType v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasType() => $_has(2);
  @$pb.TagNumber(3)
  void clearType() => clearField(3);

  /// Optional. The time zone of the device. If omitted, device returns
  /// timestamps in local time
  @$pb.TagNumber(4)
  $core.String get timeZoneId => $_getSZ(3);
  @$pb.TagNumber(4)
  set timeZoneId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTimeZoneId() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimeZoneId() => clearField(4);

  /// Custom unique external id
  @$pb.TagNumber(5)
  $core.String get externalId => $_getSZ(4);
  @$pb.TagNumber(5)
  set externalId($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasExternalId() => $_has(4);
  @$pb.TagNumber(5)
  void clearExternalId() => clearField(5);

  @$pb.TagNumber(10)
  $core.List<DeviceAction> get actions => $_getList(5);
}

enum DeviceAction_Configuration {
  clock, 
  custom, 
  notSet
}

class DeviceAction extends $pb.GeneratedMessage {
  factory DeviceAction({
    $core.int? id,
    $core.int? deviceActionId,
    $core.String? icon,
    $core.String? name,
    DeviceActionType? type,
    ClockConfiguration? clock,
    CustomConfiguration? custom,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (deviceActionId != null) {
      result.deviceActionId = deviceActionId;
    }
    if (icon != null) {
      result.icon = icon;
    }
    if (name != null) {
      result.name = name;
    }
    if (type != null) {
      result.type = type;
    }
    if (clock != null) {
      result.clock = clock;
    }
    if (custom != null) {
      result.custom = custom;
    }
    return result;
  }
  DeviceAction._() : super();
  factory DeviceAction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DeviceAction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, DeviceAction_Configuration> _DeviceAction_ConfigurationByTag = {
    100 : DeviceAction_Configuration.clock,
    101 : DeviceAction_Configuration.custom,
    0 : DeviceAction_Configuration.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'DeviceAction', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.devices'), createEmptyInstance: create)
    ..oo(0, [100, 101])
    ..a<$core.int>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.O3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'deviceActionId', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'icon')
    ..aOS(4, _omitFieldNames ? '' : 'name')
    ..e<DeviceActionType>(5, _omitFieldNames ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: DeviceActionType.CLOCK, valueOf: DeviceActionType.valueOf, enumValues: DeviceActionType.values)
    ..aOM<ClockConfiguration>(100, _omitFieldNames ? '' : 'clock', subBuilder: ClockConfiguration.create)
    ..aOM<CustomConfiguration>(101, _omitFieldNames ? '' : 'custom', subBuilder: CustomConfiguration.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DeviceAction clone() => DeviceAction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DeviceAction copyWith(void Function(DeviceAction) updates) => super.copyWith((message) => updates(message as DeviceAction)) as DeviceAction;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeviceAction create() => DeviceAction._();
  DeviceAction createEmptyInstance() => create();
  static $pb.PbList<DeviceAction> createRepeated() => $pb.PbList<DeviceAction>();
  @$core.pragma('dart2js:noInline')
  static DeviceAction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeviceAction>(create);
  static DeviceAction? _defaultInstance;

  DeviceAction_Configuration whichConfiguration() => _DeviceAction_ConfigurationByTag[$_whichOneof(0)]!;
  void clearConfiguration() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// e.g. DataFox record type (an int from 1-6 for the button position)
  @$pb.TagNumber(2)
  $core.int get deviceActionId => $_getIZ(1);
  @$pb.TagNumber(2)
  set deviceActionId($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDeviceActionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearDeviceActionId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get icon => $_getSZ(2);
  @$pb.TagNumber(3)
  set icon($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasIcon() => $_has(2);
  @$pb.TagNumber(3)
  void clearIcon() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get name => $_getSZ(3);
  @$pb.TagNumber(4)
  set name($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasName() => $_has(3);
  @$pb.TagNumber(4)
  void clearName() => clearField(4);

  @$pb.TagNumber(5)
  DeviceActionType get type => $_getN(4);
  @$pb.TagNumber(5)
  set type(DeviceActionType v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasType() => $_has(4);
  @$pb.TagNumber(5)
  void clearType() => clearField(5);

  @$pb.TagNumber(100)
  ClockConfiguration get clock => $_getN(5);
  @$pb.TagNumber(100)
  set clock(ClockConfiguration v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasClock() => $_has(5);
  @$pb.TagNumber(100)
  void clearClock() => clearField(100);
  @$pb.TagNumber(100)
  ClockConfiguration ensureClock() => $_ensure(5);

  @$pb.TagNumber(101)
  CustomConfiguration get custom => $_getN(6);
  @$pb.TagNumber(101)
  set custom(CustomConfiguration v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasCustom() => $_has(6);
  @$pb.TagNumber(101)
  void clearCustom() => clearField(101);
  @$pb.TagNumber(101)
  CustomConfiguration ensureCustom() => $_ensure(6);
}

class ClockConfiguration extends $pb.GeneratedMessage {
  factory ClockConfiguration({
    $core.String? timeTypeId,
    $core.String? costCenterId,
    $core.String? projectId,
    $core.String? projectTaskId,
  }) {
    final result = create();
    if (timeTypeId != null) {
      result.timeTypeId = timeTypeId;
    }
    if (costCenterId != null) {
      result.costCenterId = costCenterId;
    }
    if (projectId != null) {
      result.projectId = projectId;
    }
    if (projectTaskId != null) {
      result.projectTaskId = projectTaskId;
    }
    return result;
  }
  ClockConfiguration._() : super();
  factory ClockConfiguration.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ClockConfiguration.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ClockConfiguration', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.devices'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'timeTypeId')
    ..aOS(2, _omitFieldNames ? '' : 'costCenterId')
    ..aOS(3, _omitFieldNames ? '' : 'projectId')
    ..aOS(4, _omitFieldNames ? '' : 'projectTaskId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ClockConfiguration clone() => ClockConfiguration()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ClockConfiguration copyWith(void Function(ClockConfiguration) updates) => super.copyWith((message) => updates(message as ClockConfiguration)) as ClockConfiguration;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ClockConfiguration create() => ClockConfiguration._();
  ClockConfiguration createEmptyInstance() => create();
  static $pb.PbList<ClockConfiguration> createRepeated() => $pb.PbList<ClockConfiguration>();
  @$core.pragma('dart2js:noInline')
  static ClockConfiguration getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClockConfiguration>(create);
  static ClockConfiguration? _defaultInstance;

  /// Default configured parameters to clock on
  @$pb.TagNumber(1)
  $core.String get timeTypeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set timeTypeId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimeTypeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimeTypeId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get costCenterId => $_getSZ(1);
  @$pb.TagNumber(2)
  set costCenterId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCostCenterId() => $_has(1);
  @$pb.TagNumber(2)
  void clearCostCenterId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get projectId => $_getSZ(2);
  @$pb.TagNumber(3)
  set projectId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasProjectId() => $_has(2);
  @$pb.TagNumber(3)
  void clearProjectId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get projectTaskId => $_getSZ(3);
  @$pb.TagNumber(4)
  set projectTaskId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasProjectTaskId() => $_has(3);
  @$pb.TagNumber(4)
  void clearProjectTaskId() => clearField(4);
}

class CustomConfiguration extends $pb.GeneratedMessage {
  factory CustomConfiguration({
    $core.String? operation,
  }) {
    final result = create();
    if (operation != null) {
      result.operation = operation;
    }
    return result;
  }
  CustomConfiguration._() : super();
  factory CustomConfiguration.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CustomConfiguration.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CustomConfiguration', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.devices'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'operation')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CustomConfiguration clone() => CustomConfiguration()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CustomConfiguration copyWith(void Function(CustomConfiguration) updates) => super.copyWith((message) => updates(message as CustomConfiguration)) as CustomConfiguration;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CustomConfiguration create() => CustomConfiguration._();
  CustomConfiguration createEmptyInstance() => create();
  static $pb.PbList<CustomConfiguration> createRepeated() => $pb.PbList<CustomConfiguration>();
  @$core.pragma('dart2js:noInline')
  static CustomConfiguration getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CustomConfiguration>(create);
  static CustomConfiguration? _defaultInstance;

  /// the operation that should be carried out by the device process
  @$pb.TagNumber(1)
  $core.String get operation => $_getSZ(0);
  @$pb.TagNumber(1)
  set operation($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOperation() => $_has(0);
  @$pb.TagNumber(1)
  void clearOperation() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
