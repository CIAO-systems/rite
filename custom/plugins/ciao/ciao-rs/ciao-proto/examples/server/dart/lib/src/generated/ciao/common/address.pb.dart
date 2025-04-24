//
//  Generated code. Do not modify.
//  source: ciao/common/address.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Address information
class Address extends $pb.GeneratedMessage {
  factory Address({
    $core.String? city,
    $core.String? postalCode,
    $core.String? addressLine1,
    $core.String? regionCode,
    $core.String? state,
    $core.String? addressLine2,
  }) {
    final result = create();
    if (city != null) {
      result.city = city;
    }
    if (postalCode != null) {
      result.postalCode = postalCode;
    }
    if (addressLine1 != null) {
      result.addressLine1 = addressLine1;
    }
    if (regionCode != null) {
      result.regionCode = regionCode;
    }
    if (state != null) {
      result.state = state;
    }
    if (addressLine2 != null) {
      result.addressLine2 = addressLine2;
    }
    return result;
  }
  Address._() : super();
  factory Address.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Address.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Address', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.common'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'city')
    ..aOS(2, _omitFieldNames ? '' : 'postalCode')
    ..aOS(3, _omitFieldNames ? '' : 'addressLine1', protoName: 'address_line_1')
    ..aOS(4, _omitFieldNames ? '' : 'regionCode')
    ..aOS(5, _omitFieldNames ? '' : 'state')
    ..aOS(6, _omitFieldNames ? '' : 'addressLine2', protoName: 'address_line_2')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Address clone() => Address()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Address copyWith(void Function(Address) updates) => super.copyWith((message) => updates(message as Address)) as Address;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Address create() => Address._();
  Address createEmptyInstance() => create();
  static $pb.PbList<Address> createRepeated() => $pb.PbList<Address>();
  @$core.pragma('dart2js:noInline')
  static Address getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Address>(create);
  static Address? _defaultInstance;

  /// Name of the city
  @$pb.TagNumber(1)
  $core.String get city => $_getSZ(0);
  @$pb.TagNumber(1)
  set city($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCity() => $_has(0);
  @$pb.TagNumber(1)
  void clearCity() => clearField(1);

  /// Postalcode (or ZIP) code
  @$pb.TagNumber(2)
  $core.String get postalCode => $_getSZ(1);
  @$pb.TagNumber(2)
  set postalCode($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPostalCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearPostalCode() => clearField(2);

  /// Name of the street including house number
  /// optionally with additional information like floor or apartment number
  @$pb.TagNumber(3)
  $core.String get addressLine1 => $_getSZ(2);
  @$pb.TagNumber(3)
  set addressLine1($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAddressLine1() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddressLine1() => clearField(3);

  /// Country name (e.g. DE for Germany)
  @$pb.TagNumber(4)
  $core.String get regionCode => $_getSZ(3);
  @$pb.TagNumber(4)
  set regionCode($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRegionCode() => $_has(3);
  @$pb.TagNumber(4)
  void clearRegionCode() => clearField(4);

  /// State or province (e.g. Bavaria or California)
  @$pb.TagNumber(5)
  $core.String get state => $_getSZ(4);
  @$pb.TagNumber(5)
  set state($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasState() => $_has(4);
  @$pb.TagNumber(5)
  void clearState() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get addressLine2 => $_getSZ(5);
  @$pb.TagNumber(6)
  set addressLine2($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasAddressLine2() => $_has(5);
  @$pb.TagNumber(6)
  void clearAddressLine2() => clearField(6);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
