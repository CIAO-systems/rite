//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/service.proto
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

import 'service.pb.dart' as $6;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.time_type.TimeTypeService')
class TimeTypeServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$6.GetRequest, $6.GetResponse>(
      '/ciao.time_tracking.time_type.TimeTypeService/Get',
      ($6.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $6.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$6.ListRequest, $6.ListResponse>(
      '/ciao.time_tracking.time_type.TimeTypeService/List',
      ($6.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $6.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$6.CreateRequest, $6.CreateResponse>(
      '/ciao.time_tracking.time_type.TimeTypeService/Create',
      ($6.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $6.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$6.UpdateRequest, $6.UpdateResponse>(
      '/ciao.time_tracking.time_type.TimeTypeService/Update',
      ($6.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $6.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$6.DeleteRequest, $6.DeleteResponse>(
      '/ciao.time_tracking.time_type.TimeTypeService/Delete',
      ($6.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $6.DeleteResponse.fromBuffer(value));

  TimeTypeServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$6.GetResponse> get($6.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$6.ListResponse> list($6.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$6.CreateResponse> create($6.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$6.UpdateResponse> update($6.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$6.DeleteResponse> delete($6.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.time_type.TimeTypeService')
abstract class TimeTypeServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.time_type.TimeTypeService';

  TimeTypeServiceBase() {
    $addMethod($grpc.ServiceMethod<$6.GetRequest, $6.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $6.GetRequest.fromBuffer(value),
        ($6.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$6.ListRequest, $6.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $6.ListRequest.fromBuffer(value),
        ($6.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$6.CreateRequest, $6.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $6.CreateRequest.fromBuffer(value),
        ($6.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$6.UpdateRequest, $6.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $6.UpdateRequest.fromBuffer(value),
        ($6.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$6.DeleteRequest, $6.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $6.DeleteRequest.fromBuffer(value),
        ($6.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$6.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$6.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$6.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$6.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$6.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$6.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$6.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$6.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$6.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$6.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$6.GetResponse> get($grpc.ServiceCall call, $6.GetRequest request);
  $async.Stream<$6.ListResponse> list($grpc.ServiceCall call, $6.ListRequest request);
  $async.Future<$6.CreateResponse> create($grpc.ServiceCall call, $6.CreateRequest request);
  $async.Future<$6.UpdateResponse> update($grpc.ServiceCall call, $6.UpdateRequest request);
  $async.Future<$6.DeleteResponse> delete($grpc.ServiceCall call, $6.DeleteRequest request);
}
