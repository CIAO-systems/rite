//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/time_type/group/service.proto
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

import 'service.pb.dart' as $7;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.time_type.group.TimeTypeGroupService')
class TimeTypeGroupServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$7.GetRequest, $7.GetResponse>(
      '/ciao.time_tracking.time_type.group.TimeTypeGroupService/Get',
      ($7.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $7.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$7.ListRequest, $7.ListResponse>(
      '/ciao.time_tracking.time_type.group.TimeTypeGroupService/List',
      ($7.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $7.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$7.CreateRequest, $7.CreateResponse>(
      '/ciao.time_tracking.time_type.group.TimeTypeGroupService/Create',
      ($7.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $7.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$7.UpdateRequest, $7.UpdateResponse>(
      '/ciao.time_tracking.time_type.group.TimeTypeGroupService/Update',
      ($7.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $7.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$7.DeleteRequest, $7.DeleteResponse>(
      '/ciao.time_tracking.time_type.group.TimeTypeGroupService/Delete',
      ($7.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $7.DeleteResponse.fromBuffer(value));

  TimeTypeGroupServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$7.GetResponse> get($7.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$7.ListResponse> list($7.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$7.CreateResponse> create($7.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$7.UpdateResponse> update($7.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$7.DeleteResponse> delete($7.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.time_type.group.TimeTypeGroupService')
abstract class TimeTypeGroupServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.time_type.group.TimeTypeGroupService';

  TimeTypeGroupServiceBase() {
    $addMethod($grpc.ServiceMethod<$7.GetRequest, $7.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $7.GetRequest.fromBuffer(value),
        ($7.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$7.ListRequest, $7.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $7.ListRequest.fromBuffer(value),
        ($7.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$7.CreateRequest, $7.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $7.CreateRequest.fromBuffer(value),
        ($7.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$7.UpdateRequest, $7.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $7.UpdateRequest.fromBuffer(value),
        ($7.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$7.DeleteRequest, $7.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $7.DeleteRequest.fromBuffer(value),
        ($7.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$7.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$7.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$7.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$7.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$7.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$7.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$7.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$7.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$7.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$7.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$7.GetResponse> get($grpc.ServiceCall call, $7.GetRequest request);
  $async.Stream<$7.ListResponse> list($grpc.ServiceCall call, $7.ListRequest request);
  $async.Future<$7.CreateResponse> create($grpc.ServiceCall call, $7.CreateRequest request);
  $async.Future<$7.UpdateResponse> update($grpc.ServiceCall call, $7.UpdateRequest request);
  $async.Future<$7.DeleteResponse> delete($grpc.ServiceCall call, $7.DeleteRequest request);
}
