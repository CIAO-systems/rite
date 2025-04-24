//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/group/time_type_group.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class TimeTypeGroup extends $pb.GeneratedMessage {
  factory TimeTypeGroup({
    $core.String? id,
    $core.String? name,
    $core.Iterable<$core.String>? timeTypeIds,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (name != null) {
      result.name = name;
    }
    if (timeTypeIds != null) {
      result.timeTypeIds.addAll(timeTypeIds);
    }
    return result;
  }
  TimeTypeGroup._() : super();
  factory TimeTypeGroup.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TimeTypeGroup.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TimeTypeGroup', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking.time_type.group'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..pPS(3, _omitFieldNames ? '' : 'timeTypeIds')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TimeTypeGroup clone() => TimeTypeGroup()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TimeTypeGroup copyWith(void Function(TimeTypeGroup) updates) => super.copyWith((message) => updates(message as TimeTypeGroup)) as TimeTypeGroup;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TimeTypeGroup create() => TimeTypeGroup._();
  TimeTypeGroup createEmptyInstance() => create();
  static $pb.PbList<TimeTypeGroup> createRepeated() => $pb.PbList<TimeTypeGroup>();
  @$core.pragma('dart2js:noInline')
  static TimeTypeGroup getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TimeTypeGroup>(create);
  static TimeTypeGroup? _defaultInstance;

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
  $core.List<$core.String> get timeTypeIds => $_getList(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
