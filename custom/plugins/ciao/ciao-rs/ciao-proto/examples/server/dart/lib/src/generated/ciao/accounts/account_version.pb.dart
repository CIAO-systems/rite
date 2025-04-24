//
//  Generated code. Do not modify.
//  source: ciao/accounts/account_version.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../common/address.pb.dart' as $15;
import '../common/timestamp.pb.dart' as $13;

/// Account version data
class AccountVersion extends $pb.GeneratedMessage {
  factory AccountVersion({
    $core.String? id,
    $core.String? userId,
    $15.Address? address,
    $13.Timestamp? date,
    $13.Timestamp? customEntitlementResetDate,
    $core.bool? customEntitlementResetDateActive,
    $core.String? holidayCalendarId,
    $core.String? shiftScheduleId,
    $core.String? vacationBalanceId,
    $core.Iterable<AccountCostCenter>? costCenters,
    $core.double? vacationEntitlementOffset,
    $core.double? weeklyWorkingHours,
    $core.double? yearlyVacationEntitlement,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    if (userId != null) {
      result.userId = userId;
    }
    if (address != null) {
      result.address = address;
    }
    if (date != null) {
      result.date = date;
    }
    if (customEntitlementResetDate != null) {
      result.customEntitlementResetDate = customEntitlementResetDate;
    }
    if (customEntitlementResetDateActive != null) {
      result.customEntitlementResetDateActive = customEntitlementResetDateActive;
    }
    if (holidayCalendarId != null) {
      result.holidayCalendarId = holidayCalendarId;
    }
    if (shiftScheduleId != null) {
      result.shiftScheduleId = shiftScheduleId;
    }
    if (vacationBalanceId != null) {
      result.vacationBalanceId = vacationBalanceId;
    }
    if (costCenters != null) {
      result.costCenters.addAll(costCenters);
    }
    if (vacationEntitlementOffset != null) {
      result.vacationEntitlementOffset = vacationEntitlementOffset;
    }
    if (weeklyWorkingHours != null) {
      result.weeklyWorkingHours = weeklyWorkingHours;
    }
    if (yearlyVacationEntitlement != null) {
      result.yearlyVacationEntitlement = yearlyVacationEntitlement;
    }
    return result;
  }
  AccountVersion._() : super();
  factory AccountVersion.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountVersion.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountVersion', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.accounts'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'userId')
    ..aOM<$15.Address>(3, _omitFieldNames ? '' : 'address', subBuilder: $15.Address.create)
    ..aOM<$13.Timestamp>(4, _omitFieldNames ? '' : 'date', subBuilder: $13.Timestamp.create)
    ..aOM<$13.Timestamp>(5, _omitFieldNames ? '' : 'customEntitlementResetDate', subBuilder: $13.Timestamp.create)
    ..aOB(6, _omitFieldNames ? '' : 'customEntitlementResetDateActive')
    ..aOS(7, _omitFieldNames ? '' : 'holidayCalendarId')
    ..aOS(8, _omitFieldNames ? '' : 'shiftScheduleId')
    ..aOS(9, _omitFieldNames ? '' : 'vacationBalanceId')
    ..pc<AccountCostCenter>(10, _omitFieldNames ? '' : 'costCenters', $pb.PbFieldType.PM, subBuilder: AccountCostCenter.create)
    ..a<$core.double>(16, _omitFieldNames ? '' : 'vacationEntitlementOffset', $pb.PbFieldType.OD)
    ..a<$core.double>(17, _omitFieldNames ? '' : 'weeklyWorkingHours', $pb.PbFieldType.OD)
    ..a<$core.double>(18, _omitFieldNames ? '' : 'yearlyVacationEntitlement', $pb.PbFieldType.OD)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountVersion clone() => AccountVersion()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountVersion copyWith(void Function(AccountVersion) updates) => super.copyWith((message) => updates(message as AccountVersion)) as AccountVersion;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountVersion create() => AccountVersion._();
  AccountVersion createEmptyInstance() => create();
  static $pb.PbList<AccountVersion> createRepeated() => $pb.PbList<AccountVersion>();
  @$core.pragma('dart2js:noInline')
  static AccountVersion getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountVersion>(create);
  static AccountVersion? _defaultInstance;

  /// Unique ID (UUID) of the version
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  /// Associated account ID (UUID)
  @$pb.TagNumber(2)
  $core.String get userId => $_getSZ(1);
  @$pb.TagNumber(2)
  set userId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  /// Address information of this version
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

  /// Timestamp representing the validity date
  @$pb.TagNumber(4)
  $13.Timestamp get date => $_getN(3);
  @$pb.TagNumber(4)
  set date($13.Timestamp v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasDate() => $_has(3);
  @$pb.TagNumber(4)
  void clearDate() => clearField(4);
  @$pb.TagNumber(4)
  $13.Timestamp ensureDate() => $_ensure(3);

  /// Custom entitlement reset date
  @$pb.TagNumber(5)
  $13.Timestamp get customEntitlementResetDate => $_getN(4);
  @$pb.TagNumber(5)
  set customEntitlementResetDate($13.Timestamp v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasCustomEntitlementResetDate() => $_has(4);
  @$pb.TagNumber(5)
  void clearCustomEntitlementResetDate() => clearField(5);
  @$pb.TagNumber(5)
  $13.Timestamp ensureCustomEntitlementResetDate() => $_ensure(4);

  /// Indicates if custom entitlement reset date is active
  @$pb.TagNumber(6)
  $core.bool get customEntitlementResetDateActive => $_getBF(5);
  @$pb.TagNumber(6)
  set customEntitlementResetDateActive($core.bool v) { $_setBool(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasCustomEntitlementResetDateActive() => $_has(5);
  @$pb.TagNumber(6)
  void clearCustomEntitlementResetDateActive() => clearField(6);

  /// Holiday calendar ID associated with this account version
  @$pb.TagNumber(7)
  $core.String get holidayCalendarId => $_getSZ(6);
  @$pb.TagNumber(7)
  set holidayCalendarId($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasHolidayCalendarId() => $_has(6);
  @$pb.TagNumber(7)
  void clearHolidayCalendarId() => clearField(7);

  /// Shift schedule ID associated with this account version
  @$pb.TagNumber(8)
  $core.String get shiftScheduleId => $_getSZ(7);
  @$pb.TagNumber(8)
  set shiftScheduleId($core.String v) { $_setString(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasShiftScheduleId() => $_has(7);
  @$pb.TagNumber(8)
  void clearShiftScheduleId() => clearField(8);

  /// Vacation balance ID associated with this account version
  @$pb.TagNumber(9)
  $core.String get vacationBalanceId => $_getSZ(8);
  @$pb.TagNumber(9)
  set vacationBalanceId($core.String v) { $_setString(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasVacationBalanceId() => $_has(8);
  @$pb.TagNumber(9)
  void clearVacationBalanceId() => clearField(9);

  /// List of associated cost centers
  @$pb.TagNumber(10)
  $core.List<AccountCostCenter> get costCenters => $_getList(9);

  /// Offset for yearly vacation entitlement (in days)
  @$pb.TagNumber(16)
  $core.double get vacationEntitlementOffset => $_getN(10);
  @$pb.TagNumber(16)
  set vacationEntitlementOffset($core.double v) { $_setDouble(10, v); }
  @$pb.TagNumber(16)
  $core.bool hasVacationEntitlementOffset() => $_has(10);
  @$pb.TagNumber(16)
  void clearVacationEntitlementOffset() => clearField(16);

  /// Weekly working hours
  @$pb.TagNumber(17)
  $core.double get weeklyWorkingHours => $_getN(11);
  @$pb.TagNumber(17)
  set weeklyWorkingHours($core.double v) { $_setDouble(11, v); }
  @$pb.TagNumber(17)
  $core.bool hasWeeklyWorkingHours() => $_has(11);
  @$pb.TagNumber(17)
  void clearWeeklyWorkingHours() => clearField(17);

  /// Yearly vacation entitlement (in days)
  @$pb.TagNumber(18)
  $core.double get yearlyVacationEntitlement => $_getN(12);
  @$pb.TagNumber(18)
  set yearlyVacationEntitlement($core.double v) { $_setDouble(12, v); }
  @$pb.TagNumber(18)
  $core.bool hasYearlyVacationEntitlement() => $_has(12);
  @$pb.TagNumber(18)
  void clearYearlyVacationEntitlement() => clearField(18);
}

/// Represents a cost center linked to an account version
class AccountCostCenter extends $pb.GeneratedMessage {
  factory AccountCostCenter({
    $core.String? costCenterId,
    $core.int? percentage,
  }) {
    final result = create();
    if (costCenterId != null) {
      result.costCenterId = costCenterId;
    }
    if (percentage != null) {
      result.percentage = percentage;
    }
    return result;
  }
  AccountCostCenter._() : super();
  factory AccountCostCenter.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountCostCenter.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountCostCenter', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.accounts'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'costCenterId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'percentage', $pb.PbFieldType.O3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountCostCenter clone() => AccountCostCenter()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountCostCenter copyWith(void Function(AccountCostCenter) updates) => super.copyWith((message) => updates(message as AccountCostCenter)) as AccountCostCenter;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountCostCenter create() => AccountCostCenter._();
  AccountCostCenter createEmptyInstance() => create();
  static $pb.PbList<AccountCostCenter> createRepeated() => $pb.PbList<AccountCostCenter>();
  @$core.pragma('dart2js:noInline')
  static AccountCostCenter getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountCostCenter>(create);
  static AccountCostCenter? _defaultInstance;

  /// Cost center ID
  @$pb.TagNumber(1)
  $core.String get costCenterId => $_getSZ(0);
  @$pb.TagNumber(1)
  set costCenterId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCostCenterId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCostCenterId() => clearField(1);

  /// Percentage allocation of this cost center
  @$pb.TagNumber(2)
  $core.int get percentage => $_getIZ(1);
  @$pb.TagNumber(2)
  set percentage($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPercentage() => $_has(1);
  @$pb.TagNumber(2)
  void clearPercentage() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
