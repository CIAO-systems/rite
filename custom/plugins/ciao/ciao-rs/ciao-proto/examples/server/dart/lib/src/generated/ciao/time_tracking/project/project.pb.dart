//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/project/project.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../../common/timestamp.pb.dart' as $13;

/// Project data
class Project extends $pb.GeneratedMessage {
  factory Project({
    $core.String? id,
    $core.String? externalId,
    $core.String? name,
    $13.Timestamp? startDate,
    $13.Timestamp? endDate,
    $13.Timestamp? closedDate,
    $core.String? parentId,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (externalId != null) {
      result.externalId = externalId;
    }
    if (name != null) {
      result.name = name;
    }
    if (startDate != null) {
      result.startDate = startDate;
    }
    if (endDate != null) {
      result.endDate = endDate;
    }
    if (closedDate != null) {
      result.closedDate = closedDate;
    }
    if (parentId != null) {
      result.parentId = parentId;
    }
    return result;
  }
  Project._() : super();
  factory Project.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Project.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Project', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking.project'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'externalId')
    ..aOS(3, _omitFieldNames ? '' : 'name')
    ..aOM<$13.Timestamp>(4, _omitFieldNames ? '' : 'startDate', subBuilder: $13.Timestamp.create)
    ..aOM<$13.Timestamp>(5, _omitFieldNames ? '' : 'endDate', subBuilder: $13.Timestamp.create)
    ..aOM<$13.Timestamp>(6, _omitFieldNames ? '' : 'closedDate', subBuilder: $13.Timestamp.create)
    ..aOS(7, _omitFieldNames ? '' : 'parentId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Project clone() => Project()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Project copyWith(void Function(Project) updates) => super.copyWith((message) => updates(message as Project)) as Project;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Project create() => Project._();
  Project createEmptyInstance() => create();
  static $pb.PbList<Project> createRepeated() => $pb.PbList<Project>();
  @$core.pragma('dart2js:noInline')
  static Project getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Project>(create);
  static Project? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get externalId => $_getSZ(1);
  @$pb.TagNumber(2)
  set externalId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasExternalId() => $_has(1);
  @$pb.TagNumber(2)
  void clearExternalId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(3)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(3)
  void clearName() => clearField(3);

  @$pb.TagNumber(4)
  $13.Timestamp get startDate => $_getN(3);
  @$pb.TagNumber(4)
  set startDate($13.Timestamp v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasStartDate() => $_has(3);
  @$pb.TagNumber(4)
  void clearStartDate() => clearField(4);
  @$pb.TagNumber(4)
  $13.Timestamp ensureStartDate() => $_ensure(3);

  @$pb.TagNumber(5)
  $13.Timestamp get endDate => $_getN(4);
  @$pb.TagNumber(5)
  set endDate($13.Timestamp v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasEndDate() => $_has(4);
  @$pb.TagNumber(5)
  void clearEndDate() => clearField(5);
  @$pb.TagNumber(5)
  $13.Timestamp ensureEndDate() => $_ensure(4);

  @$pb.TagNumber(6)
  $13.Timestamp get closedDate => $_getN(5);
  @$pb.TagNumber(6)
  set closedDate($13.Timestamp v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasClosedDate() => $_has(5);
  @$pb.TagNumber(6)
  void clearClosedDate() => clearField(6);
  @$pb.TagNumber(6)
  $13.Timestamp ensureClosedDate() => $_ensure(5);

  @$pb.TagNumber(7)
  $core.String get parentId => $_getSZ(6);
  @$pb.TagNumber(7)
  set parentId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasParentId() => $_has(6);
  @$pb.TagNumber(7)
  void clearParentId() => clearField(7);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
