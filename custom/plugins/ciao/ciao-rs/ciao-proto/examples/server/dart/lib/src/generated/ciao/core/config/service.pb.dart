//
//  Generated code. Do not modify.
//  source: ciao/core/config/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Request to return configuration values for the given `keys`. If `keys` is
/// empty, all known configuration values should be returned
class ConfigurationValuesRequest extends $pb.GeneratedMessage {
  factory ConfigurationValuesRequest({
    $core.Iterable<$core.String>? keys,
  }) {
    final result = create();
    if (keys != null) {
      result.keys.addAll(keys);
    }
    return result;
  }
  ConfigurationValuesRequest._() : super();
  factory ConfigurationValuesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConfigurationValuesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConfigurationValuesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.core.config'), createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'keys')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConfigurationValuesRequest clone() => ConfigurationValuesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConfigurationValuesRequest copyWith(void Function(ConfigurationValuesRequest) updates) => super.copyWith((message) => updates(message as ConfigurationValuesRequest)) as ConfigurationValuesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConfigurationValuesRequest create() => ConfigurationValuesRequest._();
  ConfigurationValuesRequest createEmptyInstance() => create();
  static $pb.PbList<ConfigurationValuesRequest> createRepeated() => $pb.PbList<ConfigurationValuesRequest>();
  @$core.pragma('dart2js:noInline')
  static ConfigurationValuesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConfigurationValuesRequest>(create);
  static ConfigurationValuesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get keys => $_getList(0);
}

/// A map of the configuration values requested
class ConfigurationValuesResponse extends $pb.GeneratedMessage {
  factory ConfigurationValuesResponse({
    $core.Map<$core.String, $core.String>? entries,
  }) {
    final result = create();
    if (entries != null) {
      result.entries.addAll(entries);
    }
    return result;
  }
  ConfigurationValuesResponse._() : super();
  factory ConfigurationValuesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConfigurationValuesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ConfigurationValuesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.core.config'), createEmptyInstance: create)
    ..m<$core.String, $core.String>(1, _omitFieldNames ? '' : 'entries', entryClassName: 'ConfigurationValuesResponse.EntriesEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('ciao.core.config'))
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ConfigurationValuesResponse clone() => ConfigurationValuesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ConfigurationValuesResponse copyWith(void Function(ConfigurationValuesResponse) updates) => super.copyWith((message) => updates(message as ConfigurationValuesResponse)) as ConfigurationValuesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConfigurationValuesResponse create() => ConfigurationValuesResponse._();
  ConfigurationValuesResponse createEmptyInstance() => create();
  static $pb.PbList<ConfigurationValuesResponse> createRepeated() => $pb.PbList<ConfigurationValuesResponse>();
  @$core.pragma('dart2js:noInline')
  static ConfigurationValuesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConfigurationValuesResponse>(create);
  static ConfigurationValuesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.Map<$core.String, $core.String> get entries => $_getMap(0);
}

class ServerInfoRequest extends $pb.GeneratedMessage {
  factory ServerInfoRequest() => create();
  ServerInfoRequest._() : super();
  factory ServerInfoRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ServerInfoRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ServerInfoRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.core.config'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ServerInfoRequest clone() => ServerInfoRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ServerInfoRequest copyWith(void Function(ServerInfoRequest) updates) => super.copyWith((message) => updates(message as ServerInfoRequest)) as ServerInfoRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ServerInfoRequest create() => ServerInfoRequest._();
  ServerInfoRequest createEmptyInstance() => create();
  static $pb.PbList<ServerInfoRequest> createRepeated() => $pb.PbList<ServerInfoRequest>();
  @$core.pragma('dart2js:noInline')
  static ServerInfoRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerInfoRequest>(create);
  static ServerInfoRequest? _defaultInstance;
}

class ServerInfoResponse extends $pb.GeneratedMessage {
  factory ServerInfoResponse({
    $core.String? version,
    $core.String? buildTime,
  }) {
    final result = create();
    if (version != null) {
      result.version = version;
    }
    if (buildTime != null) {
      result.buildTime = buildTime;
    }
    return result;
  }
  ServerInfoResponse._() : super();
  factory ServerInfoResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ServerInfoResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ServerInfoResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'ciao.core.config'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'version')
    ..aOS(2, _omitFieldNames ? '' : 'buildTime')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ServerInfoResponse clone() => ServerInfoResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ServerInfoResponse copyWith(void Function(ServerInfoResponse) updates) => super.copyWith((message) => updates(message as ServerInfoResponse)) as ServerInfoResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ServerInfoResponse create() => ServerInfoResponse._();
  ServerInfoResponse createEmptyInstance() => create();
  static $pb.PbList<ServerInfoResponse> createRepeated() => $pb.PbList<ServerInfoResponse>();
  @$core.pragma('dart2js:noInline')
  static ServerInfoResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ServerInfoResponse>(create);
  static ServerInfoResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get version => $_getSZ(0);
  @$pb.TagNumber(1)
  set version($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasVersion() => $_has(0);
  @$pb.TagNumber(1)
  void clearVersion() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get buildTime => $_getSZ(1);
  @$pb.TagNumber(2)
  set buildTime($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasBuildTime() => $_has(1);
  @$pb.TagNumber(2)
  void clearBuildTime() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
