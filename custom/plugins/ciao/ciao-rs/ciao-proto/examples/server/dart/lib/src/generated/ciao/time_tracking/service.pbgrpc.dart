//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/service.proto
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

import 'clock_record.pb.dart' as $5;
import 'service.pb.dart' as $4;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.TimeTrackingService')
class TimeTrackingServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$4.GetRequest, $4.GetResponse>(
      '/ciao.time_tracking.TimeTrackingService/Get',
      ($4.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$4.ListRequest, $4.ListResponse>(
      '/ciao.time_tracking.TimeTrackingService/List',
      ($4.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.ListResponse.fromBuffer(value));
  static final _$clock = $grpc.ClientMethod<$5.ClockRecord, $4.ClockResponse>(
      '/ciao.time_tracking.TimeTrackingService/Clock',
      ($5.ClockRecord value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.ClockResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$4.UpdateRequest, $4.UpdateResponse>(
      '/ciao.time_tracking.TimeTrackingService/Update',
      ($4.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.UpdateResponse.fromBuffer(value));
  static final _$getUserInfo = $grpc.ClientMethod<$4.UserInfoRequest, $4.UserInfoResponse>(
      '/ciao.time_tracking.TimeTrackingService/GetUserInfo',
      ($4.UserInfoRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.UserInfoResponse.fromBuffer(value));

  TimeTrackingServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$4.GetResponse> get($4.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$4.ListResponse> list($4.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$4.ClockResponse> clock($5.ClockRecord request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$clock, request, options: options);
  }

  $grpc.ResponseFuture<$4.UpdateResponse> update($4.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$4.UserInfoResponse> getUserInfo($4.UserInfoRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getUserInfo, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.TimeTrackingService')
abstract class TimeTrackingServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.TimeTrackingService';

  TimeTrackingServiceBase() {
    $addMethod($grpc.ServiceMethod<$4.GetRequest, $4.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $4.GetRequest.fromBuffer(value),
        ($4.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$4.ListRequest, $4.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $4.ListRequest.fromBuffer(value),
        ($4.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$5.ClockRecord, $4.ClockResponse>(
        'Clock',
        clock_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $5.ClockRecord.fromBuffer(value),
        ($4.ClockResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$4.UpdateRequest, $4.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $4.UpdateRequest.fromBuffer(value),
        ($4.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$4.UserInfoRequest, $4.UserInfoResponse>(
        'GetUserInfo',
        getUserInfo_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $4.UserInfoRequest.fromBuffer(value),
        ($4.UserInfoResponse value) => value.writeToBuffer()));
  }

  $async.Future<$4.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$4.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$4.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$4.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$4.ClockResponse> clock_Pre($grpc.ServiceCall call, $async.Future<$5.ClockRecord> request) async {
    return clock(call, await request);
  }

  $async.Future<$4.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$4.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$4.UserInfoResponse> getUserInfo_Pre($grpc.ServiceCall call, $async.Future<$4.UserInfoRequest> request) async {
    return getUserInfo(call, await request);
  }

  $async.Future<$4.GetResponse> get($grpc.ServiceCall call, $4.GetRequest request);
  $async.Stream<$4.ListResponse> list($grpc.ServiceCall call, $4.ListRequest request);
  $async.Future<$4.ClockResponse> clock($grpc.ServiceCall call, $5.ClockRecord request);
  $async.Future<$4.UpdateResponse> update($grpc.ServiceCall call, $4.UpdateRequest request);
  $async.Future<$4.UserInfoResponse> getUserInfo($grpc.ServiceCall call, $4.UserInfoRequest request);
}
