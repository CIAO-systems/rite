//
//  Generated code. Do not modify.
//  source: ciao/common/time_range.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'timestamp.pb.dart' as $13;

class TimeRange extends $pb.GeneratedMessage {
  factory TimeRange({
    $13.Timestamp? startTime,
    $13.Timestamp? endTime,
  }) {
    final result = create();
    if (startTime != null) {
      result.startTime = startTime;
    }
    if (endTime != null) {
      result.endTime = endTime;
    }
    return result;
  }
  TimeRange._() : super();
  factory TimeRange.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TimeRange.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TimeRange', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.common'), createEmptyInstance: create)
    ..aOM<$13.Timestamp>(1, _omitFieldNames ? '' : 'startTime', subBuilder: $13.Timestamp.create)
    ..aOM<$13.Timestamp>(2, _omitFieldNames ? '' : 'endTime', subBuilder: $13.Timestamp.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TimeRange clone() => TimeRange()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TimeRange copyWith(void Function(TimeRange) updates) => super.copyWith((message) => updates(message as TimeRange)) as TimeRange;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TimeRange create() => TimeRange._();
  TimeRange createEmptyInstance() => create();
  static $pb.PbList<TimeRange> createRepeated() => $pb.PbList<TimeRange>();
  @$core.pragma('dart2js:noInline')
  static TimeRange getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TimeRange>(create);
  static TimeRange? _defaultInstance;

  @$pb.TagNumber(1)
  $13.Timestamp get startTime => $_getN(0);
  @$pb.TagNumber(1)
  set startTime($13.Timestamp v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartTime() => clearField(1);
  @$pb.TagNumber(1)
  $13.Timestamp ensureStartTime() => $_ensure(0);

  @$pb.TagNumber(2)
  $13.Timestamp get endTime => $_getN(1);
  @$pb.TagNumber(2)
  set endTime($13.Timestamp v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasEndTime() => $_has(1);
  @$pb.TagNumber(2)
  void clearEndTime() => clearField(2);
  @$pb.TagNumber(2)
  $13.Timestamp ensureEndTime() => $_ensure(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
