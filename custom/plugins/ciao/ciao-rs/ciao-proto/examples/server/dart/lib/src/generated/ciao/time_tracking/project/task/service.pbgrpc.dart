//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/project/task/service.proto
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

import 'service.pb.dart' as $11;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.project.task.ProjectTaskService')
class ProjectTaskServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$11.GetRequest, $11.GetResponse>(
      '/ciao.time_tracking.project.task.ProjectTaskService/Get',
      ($11.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $11.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$11.ListRequest, $11.ListResponse>(
      '/ciao.time_tracking.project.task.ProjectTaskService/List',
      ($11.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $11.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$11.CreateRequest, $11.CreateResponse>(
      '/ciao.time_tracking.project.task.ProjectTaskService/Create',
      ($11.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $11.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$11.UpdateRequest, $11.UpdateResponse>(
      '/ciao.time_tracking.project.task.ProjectTaskService/Update',
      ($11.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $11.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$11.DeleteRequest, $11.DeleteResponse>(
      '/ciao.time_tracking.project.task.ProjectTaskService/Delete',
      ($11.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $11.DeleteResponse.fromBuffer(value));

  ProjectTaskServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$11.GetResponse> get($11.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$11.ListResponse> list($11.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$11.CreateResponse> create($11.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$11.UpdateResponse> update($11.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$11.DeleteResponse> delete($11.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.project.task.ProjectTaskService')
abstract class ProjectTaskServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.project.task.ProjectTaskService';

  ProjectTaskServiceBase() {
    $addMethod($grpc.ServiceMethod<$11.GetRequest, $11.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $11.GetRequest.fromBuffer(value),
        ($11.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$11.ListRequest, $11.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $11.ListRequest.fromBuffer(value),
        ($11.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$11.CreateRequest, $11.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $11.CreateRequest.fromBuffer(value),
        ($11.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$11.UpdateRequest, $11.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $11.UpdateRequest.fromBuffer(value),
        ($11.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$11.DeleteRequest, $11.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $11.DeleteRequest.fromBuffer(value),
        ($11.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$11.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$11.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$11.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$11.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$11.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$11.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$11.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$11.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$11.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$11.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$11.GetResponse> get($grpc.ServiceCall call, $11.GetRequest request);
  $async.Stream<$11.ListResponse> list($grpc.ServiceCall call, $11.ListRequest request);
  $async.Future<$11.CreateResponse> create($grpc.ServiceCall call, $11.CreateRequest request);
  $async.Future<$11.UpdateResponse> update($grpc.ServiceCall call, $11.UpdateRequest request);
  $async.Future<$11.DeleteResponse> delete($grpc.ServiceCall call, $11.DeleteRequest request);
}
