//
//  Generated code. Do not modify.
//  source: ciao/accounts/account.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../common/address.pb.dart' as $15;
import '../common/image.pb.dart' as $16;
import '../common/name.pb.dart' as $14;

/// Account data
class Account extends $pb.GeneratedMessage {
  factory Account({
    $core.String? id,
    $14.Name? name,
    $15.Address? address,
    $16.Image? avatar,
    $core.String? email,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (name != null) {
      result.name = name;
    }
    if (address != null) {
      result.address = address;
    }
    if (avatar != null) {
      result.avatar = avatar;
    }
    if (email != null) {
      result.email = email;
    }
    return result;
  }
  Account._() : super();
  factory Account.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Account.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Account', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.accounts'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOM<$14.Name>(2, _omitFieldNames ? '' : 'name', subBuilder: $14.Name.create)
    ..aOM<$15.Address>(3, _omitFieldNames ? '' : 'address', subBuilder: $15.Address.create)
    ..aOM<$16.Image>(4, _omitFieldNames ? '' : 'avatar', subBuilder: $16.Image.create)
    ..aOS(5, _omitFieldNames ? '' : 'email')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Account clone() => Account()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Account copyWith(void Function(Account) updates) => super.copyWith((message) => updates(message as Account)) as Account;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Account create() => Account._();
  Account createEmptyInstance() => create();
  static $pb.PbList<Account> createRepeated() => $pb.PbList<Account>();
  @$core.pragma('dart2js:noInline')
  static Account getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Account>(create);
  static Account? _defaultInstance;

  /// Unique id (UUID)
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// Name of the person
  @$pb.TagNumber(2)
  $14.Name get name => $_getN(1);
  @$pb.TagNumber(2)
  set name($14.Name v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
  @$pb.TagNumber(2)
  $14.Name ensureName() => $_ensure(1);

  /// Address information of this person
  @$pb.TagNumber(3)
  $15.Address get address => $_getN(2);
  @$pb.TagNumber(3)
  set address($15.Address v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasAddress() => $_has(2);
  @$pb.TagNumber(3)
  void clearAddress() => clearField(3);
  @$pb.TagNumber(3)
  $15.Address ensureAddress() => $_ensure(2);

  /// Avatar-Image of the person
  @$pb.TagNumber(4)
  $16.Image get avatar => $_getN(3);
  @$pb.TagNumber(4)
  set avatar($16.Image v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasAvatar() => $_has(3);
  @$pb.TagNumber(4)
  void clearAvatar() => clearField(4);
  @$pb.TagNumber(4)
  $16.Image ensureAvatar() => $_ensure(3);

  /// E-mail address associated to the account
  @$pb.TagNumber(5)
  $core.String get email => $_getSZ(4);
  @$pb.TagNumber(5)
  set email($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasEmail() => $_has(4);
  @$pb.TagNumber(5)
  void clearEmail() => clearField(5);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
