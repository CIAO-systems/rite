//
//  Generated code. Do not modify.
//  source: ciao/common/timestamp.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../../google/protobuf/timestamp.pb.dart' as $12;

class Timestamp extends $pb.GeneratedMessage {
  factory Timestamp({
    $12.Timestamp? timeUtc,
    $core.String? timeZone,
  }) {
    final result = create();
    if (timeUtc != null) {
      result.timeUtc = timeUtc;
    }
    if (timeZone != null) {
      result.timeZone = timeZone;
    }
    return result;
  }
  Timestamp._() : super();
  factory Timestamp.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Timestamp.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Timestamp', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.common'), createEmptyInstance: create)
    ..aOM<$12.Timestamp>(1, _omitFieldNames ? '' : 'timeUtc', subBuilder: $12.Timestamp.create)
    ..aOS(2, _omitFieldNames ? '' : 'timeZone')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Timestamp clone() => Timestamp()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Timestamp copyWith(void Function(Timestamp) updates) => super.copyWith((message) => updates(message as Timestamp)) as Timestamp;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Timestamp create() => Timestamp._();
  Timestamp createEmptyInstance() => create();
  static $pb.PbList<Timestamp> createRepeated() => $pb.PbList<Timestamp>();
  @$core.pragma('dart2js:noInline')
  static Timestamp getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Timestamp>(create);
  static Timestamp? _defaultInstance;

  /// The time in UTC
  @$pb.TagNumber(1)
  $12.Timestamp get timeUtc => $_getN(0);
  @$pb.TagNumber(1)
  set timeUtc($12.Timestamp v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimeUtc() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimeUtc() => clearField(1);
  @$pb.TagNumber(1)
  $12.Timestamp ensureTimeUtc() => $_ensure(0);

  /// The TZ identifier
  @$pb.TagNumber(2)
  $core.String get timeZone => $_getSZ(1);
  @$pb.TagNumber(2)
  set timeZone($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTimeZone() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimeZone() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
