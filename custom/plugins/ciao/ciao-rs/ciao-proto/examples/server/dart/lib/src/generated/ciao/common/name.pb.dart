//
//  Generated code. Do not modify.
//  source: ciao/common/name.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// A combined object for all parts of a name for a person
class Name extends $pb.GeneratedMessage {
  factory Name({
    $core.String? first,
    $core.String? middle,
    $core.String? last,
  }) {
    final result = create();
    if (first != null) {
      result.first = first;
    }
    if (middle != null) {
      result.middle = middle;
    }
    if (last != null) {
      result.last = last;
    }
    return result;
  }
  Name._() : super();
  factory Name.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Name.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Name', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.common'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'first')
    ..aOS(2, _omitFieldNames ? '' : 'middle')
    ..aOS(3, _omitFieldNames ? '' : 'last')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Name clone() => Name()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Name copyWith(void Function(Name) updates) => super.copyWith((message) => updates(message as Name)) as Name;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Name create() => Name._();
  Name createEmptyInstance() => create();
  static $pb.PbList<Name> createRepeated() => $pb.PbList<Name>();
  @$core.pragma('dart2js:noInline')
  static Name getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Name>(create);
  static Name? _defaultInstance;

  /// First name
  @$pb.TagNumber(1)
  $core.String get first => $_getSZ(0);
  @$pb.TagNumber(1)
  set first($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFirst() => $_has(0);
  @$pb.TagNumber(1)
  void clearFirst() => clearField(1);

  /// Middle name (if available)
  @$pb.TagNumber(2)
  $core.String get middle => $_getSZ(1);
  @$pb.TagNumber(2)
  set middle($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMiddle() => $_has(1);
  @$pb.TagNumber(2)
  void clearMiddle() => clearField(2);

  /// Last name
  @$pb.TagNumber(3)
  $core.String get last => $_getSZ(2);
  @$pb.TagNumber(3)
  set last($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLast() => $_has(2);
  @$pb.TagNumber(3)
  void clearLast() => clearField(3);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
