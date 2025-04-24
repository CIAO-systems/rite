//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/time_type.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../../common/color.pb.dart' as $22;

/// Time type data
class TimeType extends $pb.GeneratedMessage {
  factory TimeType({
    $core.String? id,
    $core.String? name,
    $core.String? shorthand,
    $22.Color? color,
    $core.String? icon,
    TimeTypeOptions? options,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (name != null) {
      result.name = name;
    }
    if (shorthand != null) {
      result.shorthand = shorthand;
    }
    if (color != null) {
      result.color = color;
    }
    if (icon != null) {
      result.icon = icon;
    }
    if (options != null) {
      result.options = options;
    }
    return result;
  }
  TimeType._() : super();
  factory TimeType.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TimeType.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TimeType', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking.time_type'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOS(3, _omitFieldNames ? '' : 'shorthand')
    ..aOM<$22.Color>(4, _omitFieldNames ? '' : 'color', subBuilder: $22.Color.create)
    ..aOS(5, _omitFieldNames ? '' : 'icon')
    ..aOM<TimeTypeOptions>(6, _omitFieldNames ? '' : 'options', subBuilder: TimeTypeOptions.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TimeType clone() => TimeType()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TimeType copyWith(void Function(TimeType) updates) => super.copyWith((message) => updates(message as TimeType)) as TimeType;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TimeType create() => TimeType._();
  TimeType createEmptyInstance() => create();
  static $pb.PbList<TimeType> createRepeated() => $pb.PbList<TimeType>();
  @$core.pragma('dart2js:noInline')
  static TimeType getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TimeType>(create);
  static TimeType? _defaultInstance;

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
  $core.String get shorthand => $_getSZ(2);
  @$pb.TagNumber(3)
  set shorthand($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasShorthand() => $_has(2);
  @$pb.TagNumber(3)
  void clearShorthand() => clearField(3);

  @$pb.TagNumber(4)
  $22.Color get color => $_getN(3);
  @$pb.TagNumber(4)
  set color($22.Color v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasColor() => $_has(3);
  @$pb.TagNumber(4)
  void clearColor() => clearField(4);
  @$pb.TagNumber(4)
  $22.Color ensureColor() => $_ensure(3);

  @$pb.TagNumber(5)
  $core.String get icon => $_getSZ(4);
  @$pb.TagNumber(5)
  set icon($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasIcon() => $_has(4);
  @$pb.TagNumber(5)
  void clearIcon() => clearField(5);

  @$pb.TagNumber(6)
  TimeTypeOptions get options => $_getN(5);
  @$pb.TagNumber(6)
  set options(TimeTypeOptions v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasOptions() => $_has(5);
  @$pb.TagNumber(6)
  void clearOptions() => clearField(6);
  @$pb.TagNumber(6)
  TimeTypeOptions ensureOptions() => $_ensure(5);
}

class TimeTypeOptions extends $pb.GeneratedMessage {
  factory TimeTypeOptions({
    $core.bool? bookable,
    $core.bool? absence,
  }) {
    final result = create();
    if (bookable != null) {
      result.bookable = bookable;
    }
    if (absence != null) {
      result.absence = absence;
    }
    return result;
  }
  TimeTypeOptions._() : super();
  factory TimeTypeOptions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TimeTypeOptions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TimeTypeOptions', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking.time_type'), createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'bookable')
    ..aOB(2, _omitFieldNames ? '' : 'absence')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TimeTypeOptions clone() => TimeTypeOptions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TimeTypeOptions copyWith(void Function(TimeTypeOptions) updates) => super.copyWith((message) => updates(message as TimeTypeOptions)) as TimeTypeOptions;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TimeTypeOptions create() => TimeTypeOptions._();
  TimeTypeOptions createEmptyInstance() => create();
  static $pb.PbList<TimeTypeOptions> createRepeated() => $pb.PbList<TimeTypeOptions>();
  @$core.pragma('dart2js:noInline')
  static TimeTypeOptions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TimeTypeOptions>(create);
  static TimeTypeOptions? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get bookable => $_getBF(0);
  @$pb.TagNumber(1)
  set bookable($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBookable() => $_has(0);
  @$pb.TagNumber(1)
  void clearBookable() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get absence => $_getBF(1);
  @$pb.TagNumber(2)
  set absence($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAbsence() => $_has(1);
  @$pb.TagNumber(2)
  void clearAbsence() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
