//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import '../../google/protobuf/duration.pb.dart' as $21;
import '../accounts/account.pb.dart' as $17;
import '../common/time_range.pb.dart' as $20;
import 'clock_record.pb.dart' as $5;

class GetRequest extends $pb.GeneratedMessage {
  factory GetRequest({
    $core.String? id,
  }) {
    final result = create();
    if (id != null) {
      result.id = id;
    }
    return result;
  }
  GetRequest._() : super();
  factory GetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRequest clone() => GetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRequest copyWith(void Function(GetRequest) updates) => super.copyWith((message) => updates(message as GetRequest)) as GetRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRequest create() => GetRequest._();
  GetRequest createEmptyInstance() => create();
  static $pb.PbList<GetRequest> createRepeated() => $pb.PbList<GetRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRequest>(create);
  static GetRequest? _defaultInstance;

  /// Find the clock record with this unique identifier
  @$pb.TagNumber(1)
  $core.String get id => $_getSZ(0);
  @$pb.TagNumber(1)
  set id($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

class GetResponse extends $pb.GeneratedMessage {
  factory GetResponse({
    $5.ClockRecord? record,
  }) {
    final result = create();
    if (record != null) {
      result.record = record;
    }
    return result;
  }
  GetResponse._() : super();
  factory GetResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$5.ClockRecord>(1, _omitFieldNames ? '' : 'record', subBuilder: $5.ClockRecord.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetResponse clone() => GetResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetResponse copyWith(void Function(GetResponse) updates) => super.copyWith((message) => updates(message as GetResponse)) as GetResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetResponse create() => GetResponse._();
  GetResponse createEmptyInstance() => create();
  static $pb.PbList<GetResponse> createRepeated() => $pb.PbList<GetResponse>();
  @$core.pragma('dart2js:noInline')
  static GetResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetResponse>(create);
  static GetResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $5.ClockRecord get record => $_getN(0);
  @$pb.TagNumber(1)
  set record($5.ClockRecord v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => clearField(1);
  @$pb.TagNumber(1)
  $5.ClockRecord ensureRecord() => $_ensure(0);
}

/// Request parameter for TimeTrackingService.list
class ListRequest extends $pb.GeneratedMessage {
  factory ListRequest({
    $20.TimeRange? timeRange,
    $core.String? userId,
    $core.String? creatorId,
    $core.String? timeTypeId,
  }) {
    final result = create();
    if (timeRange != null) {
      result.timeRange = timeRange;
    }
    if (userId != null) {
      result.userId = userId;
    }
    if (creatorId != null) {
      result.creatorId = creatorId;
    }
    if (timeTypeId != null) {
      result.timeTypeId = timeTypeId;
    }
    return result;
  }
  ListRequest._() : super();
  factory ListRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$20.TimeRange>(1, _omitFieldNames ? '' : 'timeRange', subBuilder: $20.TimeRange.create)
    ..aOS(2, _omitFieldNames ? '' : 'userId')
    ..aOS(3, _omitFieldNames ? '' : 'creatorId')
    ..aOS(4, _omitFieldNames ? '' : 'timeTypeId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRequest clone() => ListRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRequest copyWith(void Function(ListRequest) updates) => super.copyWith((message) => updates(message as ListRequest)) as ListRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListRequest create() => ListRequest._();
  ListRequest createEmptyInstance() => create();
  static $pb.PbList<ListRequest> createRepeated() => $pb.PbList<ListRequest>();
  @$core.pragma('dart2js:noInline')
  static ListRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRequest>(create);
  static ListRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $20.TimeRange get timeRange => $_getN(0);
  @$pb.TagNumber(1)
  set timeRange($20.TimeRange v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimeRange() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimeRange() => clearField(1);
  @$pb.TagNumber(1)
  $20.TimeRange ensureTimeRange() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get userId => $_getSZ(1);
  @$pb.TagNumber(2)
  set userId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUserId() => $_has(1);
  @$pb.TagNumber(2)
  void clearUserId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get creatorId => $_getSZ(2);
  @$pb.TagNumber(3)
  set creatorId($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCreatorId() => $_has(2);
  @$pb.TagNumber(3)
  void clearCreatorId() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get timeTypeId => $_getSZ(3);
  @$pb.TagNumber(4)
  set timeTypeId($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTimeTypeId() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimeTypeId() => clearField(4);
}

class ListResponse extends $pb.GeneratedMessage {
  factory ListResponse({
    $core.Iterable<$5.ClockRecord>? records,
  }) {
    final result = create();
    if (records != null) {
      result.records.addAll(records);
    }
    return result;
  }
  ListResponse._() : super();
  factory ListResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..pc<$5.ClockRecord>(1, _omitFieldNames ? '' : 'records', $pb.PbFieldType.PM, subBuilder: $5.ClockRecord.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListResponse clone() => ListResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListResponse copyWith(void Function(ListResponse) updates) => super.copyWith((message) => updates(message as ListResponse)) as ListResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListResponse create() => ListResponse._();
  ListResponse createEmptyInstance() => create();
  static $pb.PbList<ListResponse> createRepeated() => $pb.PbList<ListResponse>();
  @$core.pragma('dart2js:noInline')
  static ListResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListResponse>(create);
  static ListResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$5.ClockRecord> get records => $_getList(0);
}

class ClockResponse extends $pb.GeneratedMessage {
  factory ClockResponse({
    $core.String? message,
    $5.ClockRecord? record,
  }) {
    final result = create();
    if (message != null) {
      result.message = message;
    }
    if (record != null) {
      result.record = record;
    }
    return result;
  }
  ClockResponse._() : super();
  factory ClockResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ClockResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ClockResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'message')
    ..aOM<$5.ClockRecord>(2, _omitFieldNames ? '' : 'record', subBuilder: $5.ClockRecord.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ClockResponse clone() => ClockResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ClockResponse copyWith(void Function(ClockResponse) updates) => super.copyWith((message) => updates(message as ClockResponse)) as ClockResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ClockResponse create() => ClockResponse._();
  ClockResponse createEmptyInstance() => create();
  static $pb.PbList<ClockResponse> createRepeated() => $pb.PbList<ClockResponse>();
  @$core.pragma('dart2js:noInline')
  static ClockResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ClockResponse>(create);
  static ClockResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get message => $_getSZ(0);
  @$pb.TagNumber(1)
  set message($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMessage() => $_has(0);
  @$pb.TagNumber(1)
  void clearMessage() => clearField(1);

  @$pb.TagNumber(2)
  $5.ClockRecord get record => $_getN(1);
  @$pb.TagNumber(2)
  set record($5.ClockRecord v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRecord() => $_has(1);
  @$pb.TagNumber(2)
  void clearRecord() => clearField(2);
  @$pb.TagNumber(2)
  $5.ClockRecord ensureRecord() => $_ensure(1);
}

/// Request parameter for TimeTrackingService.Update
class UpdateRequest extends $pb.GeneratedMessage {
  factory UpdateRequest({
    $5.ClockRecord? record,
  }) {
    final result = create();
    if (record != null) {
      result.record = record;
    }
    return result;
  }
  UpdateRequest._() : super();
  factory UpdateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UpdateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$5.ClockRecord>(1, _omitFieldNames ? '' : 'record', subBuilder: $5.ClockRecord.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UpdateRequest clone() => UpdateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UpdateRequest copyWith(void Function(UpdateRequest) updates) => super.copyWith((message) => updates(message as UpdateRequest)) as UpdateRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateRequest create() => UpdateRequest._();
  UpdateRequest createEmptyInstance() => create();
  static $pb.PbList<UpdateRequest> createRepeated() => $pb.PbList<UpdateRequest>();
  @$core.pragma('dart2js:noInline')
  static UpdateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateRequest>(create);
  static UpdateRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $5.ClockRecord get record => $_getN(0);
  @$pb.TagNumber(1)
  set record($5.ClockRecord v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => clearField(1);
  @$pb.TagNumber(1)
  $5.ClockRecord ensureRecord() => $_ensure(0);
}

class UpdateResponse extends $pb.GeneratedMessage {
  factory UpdateResponse({
    $5.ClockRecord? record,
  }) {
    final result = create();
    if (record != null) {
      result.record = record;
    }
    return result;
  }
  UpdateResponse._() : super();
  factory UpdateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UpdateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UpdateResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$5.ClockRecord>(1, _omitFieldNames ? '' : 'record', subBuilder: $5.ClockRecord.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UpdateResponse clone() => UpdateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UpdateResponse copyWith(void Function(UpdateResponse) updates) => super.copyWith((message) => updates(message as UpdateResponse)) as UpdateResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UpdateResponse create() => UpdateResponse._();
  UpdateResponse createEmptyInstance() => create();
  static $pb.PbList<UpdateResponse> createRepeated() => $pb.PbList<UpdateResponse>();
  @$core.pragma('dart2js:noInline')
  static UpdateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateResponse>(create);
  static UpdateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $5.ClockRecord get record => $_getN(0);
  @$pb.TagNumber(1)
  set record($5.ClockRecord v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRecord() => $_has(0);
  @$pb.TagNumber(1)
  void clearRecord() => clearField(1);
  @$pb.TagNumber(1)
  $5.ClockRecord ensureRecord() => $_ensure(0);
}

enum UserInfoRequest_Identity {
  userId, 
  badgeId, 
  notSet
}

class UserInfoRequest extends $pb.GeneratedMessage {
  factory UserInfoRequest({
    $core.String? userId,
    $core.String? badgeId,
  }) {
    final result = create();
    if (userId != null) {
      result.userId = userId;
    }
    if (badgeId != null) {
      result.badgeId = badgeId;
    }
    return result;
  }
  UserInfoRequest._() : super();
  factory UserInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, UserInfoRequest_Identity> _UserInfoRequest_IdentityByTag = {
    1 : UserInfoRequest_Identity.userId,
    2 : UserInfoRequest_Identity.badgeId,
    0 : UserInfoRequest_Identity.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserInfoRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOS(1, _omitFieldNames ? '' : 'userId')
    ..aOS(2, _omitFieldNames ? '' : 'badgeId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfoRequest clone() => UserInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfoRequest copyWith(void Function(UserInfoRequest) updates) => super.copyWith((message) => updates(message as UserInfoRequest)) as UserInfoRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfoRequest create() => UserInfoRequest._();
  UserInfoRequest createEmptyInstance() => create();
  static $pb.PbList<UserInfoRequest> createRepeated() => $pb.PbList<UserInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static UserInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfoRequest>(create);
  static UserInfoRequest? _defaultInstance;

  UserInfoRequest_Identity whichIdentity() => _UserInfoRequest_IdentityByTag[$_whichOneof(0)]!;
  void clearIdentity() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get userId => $_getSZ(0);
  @$pb.TagNumber(1)
  set userId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get badgeId => $_getSZ(1);
  @$pb.TagNumber(2)
  set badgeId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasBadgeId() => $_has(1);
  @$pb.TagNumber(2)
  void clearBadgeId() => clearField(2);
}

/// Information about the balance of a user
class BalanceInfo extends $pb.GeneratedMessage {
  factory BalanceInfo({
    $21.Duration? current,
  }) {
    final result = create();
    if (current != null) {
      result.current = current;
    }
    return result;
  }
  BalanceInfo._() : super();
  factory BalanceInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BalanceInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BalanceInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$21.Duration>(2, _omitFieldNames ? '' : 'current', subBuilder: $21.Duration.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BalanceInfo clone() => BalanceInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BalanceInfo copyWith(void Function(BalanceInfo) updates) => super.copyWith((message) => updates(message as BalanceInfo)) as BalanceInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BalanceInfo create() => BalanceInfo._();
  BalanceInfo createEmptyInstance() => create();
  static $pb.PbList<BalanceInfo> createRepeated() => $pb.PbList<BalanceInfo>();
  @$core.pragma('dart2js:noInline')
  static BalanceInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BalanceInfo>(create);
  static BalanceInfo? _defaultInstance;

  /// Current balance
  @$pb.TagNumber(2)
  $21.Duration get current => $_getN(0);
  @$pb.TagNumber(2)
  set current($21.Duration v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasCurrent() => $_has(0);
  @$pb.TagNumber(2)
  void clearCurrent() => clearField(2);
  @$pb.TagNumber(2)
  $21.Duration ensureCurrent() => $_ensure(0);
}

/// Information about the vacation situation of a user
class VacationInfo extends $pb.GeneratedMessage {
  factory VacationInfo({
    $21.Duration? entitlement,
    $21.Duration? remaining,
    $21.Duration? planned,
  }) {
    final result = create();
    if (entitlement != null) {
      result.entitlement = entitlement;
    }
    if (remaining != null) {
      result.remaining = remaining;
    }
    if (planned != null) {
      result.planned = planned;
    }
    return result;
  }
  VacationInfo._() : super();
  factory VacationInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VacationInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'VacationInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$21.Duration>(1, _omitFieldNames ? '' : 'entitlement', subBuilder: $21.Duration.create)
    ..aOM<$21.Duration>(2, _omitFieldNames ? '' : 'remaining', subBuilder: $21.Duration.create)
    ..aOM<$21.Duration>(3, _omitFieldNames ? '' : 'planned', subBuilder: $21.Duration.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VacationInfo clone() => VacationInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VacationInfo copyWith(void Function(VacationInfo) updates) => super.copyWith((message) => updates(message as VacationInfo)) as VacationInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static VacationInfo create() => VacationInfo._();
  VacationInfo createEmptyInstance() => create();
  static $pb.PbList<VacationInfo> createRepeated() => $pb.PbList<VacationInfo>();
  @$core.pragma('dart2js:noInline')
  static VacationInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VacationInfo>(create);
  static VacationInfo? _defaultInstance;

  /// Vacation entitlement
  @$pb.TagNumber(1)
  $21.Duration get entitlement => $_getN(0);
  @$pb.TagNumber(1)
  set entitlement($21.Duration v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasEntitlement() => $_has(0);
  @$pb.TagNumber(1)
  void clearEntitlement() => clearField(1);
  @$pb.TagNumber(1)
  $21.Duration ensureEntitlement() => $_ensure(0);

  /// Remaining vacation
  @$pb.TagNumber(2)
  $21.Duration get remaining => $_getN(1);
  @$pb.TagNumber(2)
  set remaining($21.Duration v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasRemaining() => $_has(1);
  @$pb.TagNumber(2)
  void clearRemaining() => clearField(2);
  @$pb.TagNumber(2)
  $21.Duration ensureRemaining() => $_ensure(1);

  /// Planned vacation
  @$pb.TagNumber(3)
  $21.Duration get planned => $_getN(2);
  @$pb.TagNumber(3)
  set planned($21.Duration v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasPlanned() => $_has(2);
  @$pb.TagNumber(3)
  void clearPlanned() => clearField(3);
  @$pb.TagNumber(3)
  $21.Duration ensurePlanned() => $_ensure(2);
}

class UserInfoResponse extends $pb.GeneratedMessage {
  factory UserInfoResponse({
    $17.Account? account,
    BalanceInfo? balance,
    VacationInfo? vacation,
  }) {
    final result = create();
    if (account != null) {
      result.account = account;
    }
    if (balance != null) {
      result.balance = balance;
    }
    if (vacation != null) {
      result.vacation = vacation;
    }
    return result;
  }
  UserInfoResponse._() : super();
  factory UserInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'UserInfoResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.time_tracking'), createEmptyInstance: create)
    ..aOM<$17.Account>(1, _omitFieldNames ? '' : 'account', subBuilder: $17.Account.create)
    ..aOM<BalanceInfo>(2, _omitFieldNames ? '' : 'balance', subBuilder: BalanceInfo.create)
    ..aOM<VacationInfo>(3, _omitFieldNames ? '' : 'vacation', subBuilder: VacationInfo.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserInfoResponse clone() => UserInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserInfoResponse copyWith(void Function(UserInfoResponse) updates) => super.copyWith((message) => updates(message as UserInfoResponse)) as UserInfoResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserInfoResponse create() => UserInfoResponse._();
  UserInfoResponse createEmptyInstance() => create();
  static $pb.PbList<UserInfoResponse> createRepeated() => $pb.PbList<UserInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static UserInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserInfoResponse>(create);
  static UserInfoResponse? _defaultInstance;

  /// Details for the user
  @$pb.TagNumber(1)
  $17.Account get account => $_getN(0);
  @$pb.TagNumber(1)
  set account($17.Account v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccount() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccount() => clearField(1);
  @$pb.TagNumber(1)
  $17.Account ensureAccount() => $_ensure(0);

  /// Information about the balance for this user
  @$pb.TagNumber(2)
  BalanceInfo get balance => $_getN(1);
  @$pb.TagNumber(2)
  set balance(BalanceInfo v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasBalance() => $_has(1);
  @$pb.TagNumber(2)
  void clearBalance() => clearField(2);
  @$pb.TagNumber(2)
  BalanceInfo ensureBalance() => $_ensure(1);

  /// Information about the vacation for this user
  @$pb.TagNumber(3)
  VacationInfo get vacation => $_getN(2);
  @$pb.TagNumber(3)
  set vacation(VacationInfo v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasVacation() => $_has(2);
  @$pb.TagNumber(3)
  void clearVacation() => clearField(3);
  @$pb.TagNumber(3)
  VacationInfo ensureVacation() => $_ensure(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
