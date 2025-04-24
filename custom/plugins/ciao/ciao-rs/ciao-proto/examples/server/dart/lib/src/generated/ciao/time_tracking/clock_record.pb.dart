//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/clock_record.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../common/timestamp.pb.dart' as $13;

enum ClockRecord_Identity {
  userId, 
  badgeId, 
  notSet
}

class ClockRecord extends $pb.GeneratedMessage {
  factory ClockRecord({
    $13.Timestamp? timestamp,
    $core.String? userId,
    $core.String? badgeId,
    $core.String? deviceId,
    $core.String? timeTypeId,
    $core.String? projectId,
    $core.String? costCenterId,
    $core.String? projectTaskId,
    $core.String? id,
  }) {
    final result = create();
    if (timestamp != null) {
      result.timestamp = timestamp;
    }
    if (userId != null) {
      result.userId = userId;
    }
    if (badgeId != null) {
      result.badgeId = badgeId;
    }
    if (deviceId != null) {
      result.deviceId = deviceId;
    }
    if (timeTypeId != null) {
      result.timeTypeId = timeTypeId;
    }
    if (projectId != null) {
      result.projectId = projectId;
    }
    if (costCenterId != null) {
      result.costCenterId = costCenterId;
    }
    if (projectTaskId != null) {
      result.projectTaskId = projectTaskId;
    }
    if (id != null) {
      result.id = id;
    }
    return result;
  }
  ClockRecord._() : super();
  factory ClockRecord.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ClockRecord.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ClockRecord_Identity> _ClockRecord_IdentityByTag = {
    2 : ClockRecord_Identity.userId,
    3 : ClockRecord_Identity.badgeId,
    0 : ClockRecord_Identity.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ClockRecord', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..oo(0, [2, 3])
    ..aOM<$13.Timestamp>(1, _omitFieldNames ? '' : 'timestamp', subBuilder: $13.Timestamp.create)
    ..aOS(2, _omitFieldNames ? '' : 'userId')
    ..aOS(3, _omitFieldNames ? '' : 'badgeId')
    ..aOS(4, _omitFieldNames ? '' : 'deviceId')
    ..aOS(5, _omitFieldNames ? '' : 'timeTypeId')
    ..aOS(100, _omitFieldNames ? '' : 'projectId')
    ..aOS(101, _omitFieldNames ? '' : 'costCenterId')
    ..aOS(102, _omitFieldNames ? '' : 'projectTaskId')
    ..aOS(200, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ClockRecord clone() => ClockRecord()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ClockRecord copyWith(void Function(ClockRecord) updates) => super.copyWith((message) => updates(message as ClockRecord)) as ClockRecord;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ClockRecord create() => ClockRecord._();
  ClockRecord createEmptyInstance() => create();
  static $pb.PbList<ClockRecord> createRepeated() => $pb.PbList<ClockRecord>();
  @$core.pragma('dart2js:noInline')
  static ClockRecord getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClockRecord>(create);
  static ClockRecord? _defaultInstance;

  ClockRecord_Identity whichIdentity() => _ClockRecord_IdentityByTag[$_whichOneof(0)]!;
  void clearIdentity() => clearField($_whichOneof(0));

  /// Timestamp from the terminal
  @$pb.TagNumber(1)
  $13.Timestamp get timestamp => $_getN(0);
  @$pb.TagNumber(1)
  set timestamp($13.Timestamp v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);
  @$pb.TagNumber(1)
  $13.Timestamp ensureTimestamp() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get userId => $_getSZ(1);
  @$pb.TagNumber(2)
  set userId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get badgeId => $_getSZ(2);
  @$pb.TagNumber(3)
  set badgeId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasBadgeId() => $_has(2);
  @$pb.TagNumber(3)
  void clearBadgeId() => clearField(3);

  /// The device id of the terminal (optional, if no device involved with clocking)
  @$pb.TagNumber(4)
  $core.String get deviceId => $_getSZ(3);
  @$pb.TagNumber(4)
  set deviceId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasDeviceId() => $_has(3);
  @$pb.TagNumber(4)
  void clearDeviceId() => clearField(4);

  /// The time type id to clock on
  @$pb.TagNumber(5)
  $core.String get timeTypeId => $_getSZ(4);
  @$pb.TagNumber(5)
  set timeTypeId($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasTimeTypeId() => $_has(4);
  @$pb.TagNumber(5)
  void clearTimeTypeId() => clearField(5);

  /// Optional additional data for the clock record
  @$pb.TagNumber(100)
  $core.String get projectId => $_getSZ(5);
  @$pb.TagNumber(100)
  set projectId($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(100)
  $core.bool hasProjectId() => $_has(5);
  @$pb.TagNumber(100)
  void clearProjectId() => clearField(100);

  @$pb.TagNumber(101)
  $core.String get costCenterId => $_getSZ(6);
  @$pb.TagNumber(101)
  set costCenterId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(101)
  $core.bool hasCostCenterId() => $_has(6);
  @$pb.TagNumber(101)
  void clearCostCenterId() => clearField(101);

  @$pb.TagNumber(102)
  $core.String get projectTaskId => $_getSZ(7);
  @$pb.TagNumber(102)
  set projectTaskId($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(102)
  $core.bool hasProjectTaskId() => $_has(7);
  @$pb.TagNumber(102)
  void clearProjectTaskId() => clearField(102);

  /// Unique identifier (will be set by the backend and is only relevant for queries,
  /// will be ignored when clocking)
  @$pb.TagNumber(200)
  $core.String get id => $_getSZ(8);
  @$pb.TagNumber(200)
  set id($core.String v) { $_setString(8, v); }
  @$pb.TagNumber(200)
  $core.bool hasId() => $_has(8);
  @$pb.TagNumber(200)
  void clearId() => clearField(200);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
