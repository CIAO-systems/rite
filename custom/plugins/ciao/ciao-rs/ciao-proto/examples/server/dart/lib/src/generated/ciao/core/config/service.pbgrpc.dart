//
//  Generated code. Do not modify.
//  source: ciao/core/config/service.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'service.pb.dart' as $2;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.core.config.ConfigurationService')
class ConfigurationServiceClient extends $grpc.Client {
  static final _$getConfigurationValues = $grpc.ClientMethod<$2.ConfigurationValuesRequest, $2.ConfigurationValuesResponse>(
      '/ciao.core.config.ConfigurationService/GetConfigurationValues',
      ($2.ConfigurationValuesRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $2.ConfigurationValuesResponse.fromBuffer(value));
  static final _$getServerInfo = $grpc.ClientMethod<$2.ServerInfoRequest, $2.ServerInfoResponse>(
      '/ciao.core.config.ConfigurationService/GetServerInfo',
      ($2.ServerInfoRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $2.ServerInfoResponse.fromBuffer(value));

  ConfigurationServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$2.ConfigurationValuesResponse> getConfigurationValues($2.ConfigurationValuesRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getConfigurationValues, request, options: options);
  }

  $grpc.ResponseFuture<$2.ServerInfoResponse> getServerInfo($2.ServerInfoRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getServerInfo, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.core.config.ConfigurationService')
abstract class ConfigurationServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.core.config.ConfigurationService';

  ConfigurationServiceBase() {
    $addMethod($grpc.ServiceMethod<$2.ConfigurationValuesRequest, $2.ConfigurationValuesResponse>(
        'GetConfigurationValues',
        getConfigurationValues_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $2.ConfigurationValuesRequest.fromBuffer(value),
        ($2.ConfigurationValuesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.ServerInfoRequest, $2.ServerInfoResponse>(
        'GetServerInfo',
        getServerInfo_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $2.ServerInfoRequest.fromBuffer(value),
        ($2.ServerInfoResponse value) => value.writeToBuffer()));
  }

  $async.Future<$2.ConfigurationValuesResponse> getConfigurationValues_Pre($grpc.ServiceCall call, $async.Future<$2.ConfigurationValuesRequest> request) async {
    return getConfigurationValues(call, await request);
  }

  $async.Future<$2.ServerInfoResponse> getServerInfo_Pre($grpc.ServiceCall call, $async.Future<$2.ServerInfoRequest> request) async {
    return getServerInfo(call, await request);
  }

  $async.Future<$2.ConfigurationValuesResponse> getConfigurationValues($grpc.ServiceCall call, $2.ConfigurationValuesRequest request);
  $async.Future<$2.ServerInfoResponse> getServerInfo($grpc.ServiceCall call, $2.ServerInfoRequest request);
}
