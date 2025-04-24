//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/absences/absence.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../../common/date.pb.dart' as $25;

/// Absence data
class Absence extends $pb.GeneratedMessage {
  factory Absence({
    $core.String? id,
    $25.Date? startDate,
    $25.Date? endDate,
    $core.bool? startHalfDay,
    $core.bool? endHalfDay,
    $core.String? timeTypeId,
    $core.String? userId,
    $core.bool? deleted,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (startDate != null) {
      result.startDate = startDate;
    }
    if (endDate != null) {
      result.endDate = endDate;
    }
    if (startHalfDay != null) {
      result.startHalfDay = startHalfDay;
    }
    if (endHalfDay != null) {
      result.endHalfDay = endHalfDay;
    }
    if (timeTypeId != null) {
      result.timeTypeId = timeTypeId;
    }
    if (userId != null) {
      result.userId = userId;
    }
    if (deleted != null) {
      result.deleted = deleted;
    }
    return result;
  }
  Absence._() : super();
  factory Absence.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Absence.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Absence', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking.absences'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOM<$25.Date>(2, _omitFieldNames ? '' : 'startDate', subBuilder: $25.Date.create)
    ..aOM<$25.Date>(3, _omitFieldNames ? '' : 'endDate', subBuilder: $25.Date.create)
    ..aOB(4, _omitFieldNames ? '' : 'startHalfDay')
    ..aOB(5, _omitFieldNames ? '' : 'endHalfDay')
    ..aOS(6, _omitFieldNames ? '' : 'timeTypeId')
    ..aOS(7, _omitFieldNames ? '' : 'userId')
    ..aOB(8, _omitFieldNames ? '' : 'deleted')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Absence clone() => Absence()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Absence copyWith(void Function(Absence) updates) => super.copyWith((message) => updates(message as Absence)) as Absence;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Absence create() => Absence._();
  Absence createEmptyInstance() => create();
  static $pb.PbList<Absence> createRepeated() => $pb.PbList<Absence>();
  @$core.pragma('dart2js:noInline')
  static Absence getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Absence>(create);
  static Absence? _defaultInstance;

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
  $25.Date get startDate => $_getN(1);
  @$pb.TagNumber(2)
  set startDate($25.Date v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasStartDate() => $_has(1);
  @$pb.TagNumber(2)
  void clearStartDate() => clearField(2);
  @$pb.TagNumber(2)
  $25.Date ensureStartDate() => $_ensure(1);

  @$pb.TagNumber(3)
  $25.Date get endDate => $_getN(2);
  @$pb.TagNumber(3)
  set endDate($25.Date v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasEndDate() => $_has(2);
  @$pb.TagNumber(3)
  void clearEndDate() => clearField(3);
  @$pb.TagNumber(3)
  $25.Date ensureEndDate() => $_ensure(2);

  @$pb.TagNumber(4)
  $core.bool get startHalfDay => $_getBF(3);
  @$pb.TagNumber(4)
  set startHalfDay($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasStartHalfDay() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartHalfDay() => clearField(4);

  @$pb.TagNumber(5)
  $core.bool get endHalfDay => $_getBF(4);
  @$pb.TagNumber(5)
  set endHalfDay($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasEndHalfDay() => $_has(4);
  @$pb.TagNumber(5)
  void clearEndHalfDay() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get timeTypeId => $_getSZ(5);
  @$pb.TagNumber(6)
  set timeTypeId($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasTimeTypeId() => $_has(5);
  @$pb.TagNumber(6)
  void clearTimeTypeId() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get userId => $_getSZ(6);
  @$pb.TagNumber(7)
  set userId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasUserId() => $_has(6);
  @$pb.TagNumber(7)
  void clearUserId() => clearField(7);

  @$pb.TagNumber(8)
  $core.bool get deleted => $_getBF(7);
  @$pb.TagNumber(8)
  set deleted($core.bool v) { $_setBool(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasDeleted() => $_has(7);
  @$pb.TagNumber(8)
  void clearDeleted() => clearField(8);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
