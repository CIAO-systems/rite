//
//  Generated code. Do not modify.
//  source: ciao/common/color.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Color extends $pb.GeneratedMessage {
  factory Color({
    $core.int? alpha,
    $core.int? red,
    $core.int? green,
    $core.int? blue,
  }) {
    final result = create();
    if (alpha != null) {
      result.alpha = alpha;
    }
    if (red != null) {
      result.red = red;
    }
    if (green != null) {
      result.green = green;
    }
    if (blue != null) {
      result.blue = blue;
    }
    return result;
  }
  Color._() : super();
  factory Color.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Color.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Color', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.common'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'alpha', $pb.PbFieldType.O3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'red', $pb.PbFieldType.O3)
    ..a<$core.int>(3, _omitFieldNames ? '' : 'green', $pb.PbFieldType.O3)
    ..a<$core.int>(4, _omitFieldNames ? '' : 'blue', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Color clone() => Color()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Color copyWith(void Function(Color) updates) => super.copyWith((message) => updates(message as Color)) as Color;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Color create() => Color._();
  Color createEmptyInstance() => create();
  static $pb.PbList<Color> createRepeated() => $pb.PbList<Color>();
  @$core.pragma('dart2js:noInline')
  static Color getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Color>(create);
  static Color? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get alpha => $_getIZ(0);
  @$pb.TagNumber(1)
  set alpha($core.int v) { $_setSignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAlpha() => $_has(0);
  @$pb.TagNumber(1)
  void clearAlpha() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get red => $_getIZ(1);
  @$pb.TagNumber(2)
  set red($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRed() => $_has(1);
  @$pb.TagNumber(2)
  void clearRed() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get green => $_getIZ(2);
  @$pb.TagNumber(3)
  set green($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasGreen() => $_has(2);
  @$pb.TagNumber(3)
  void clearGreen() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get blue => $_getIZ(3);
  @$pb.TagNumber(4)
  set blue($core.int v) { $_setSignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasBlue() => $_has(3);
  @$pb.TagNumber(4)
  void clearBlue() => clearField(4);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
