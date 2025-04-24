//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/project/service.proto
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

import 'service.pb.dart' as $10;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.project.ProjectService')
class ProjectServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$10.GetRequest, $10.GetResponse>(
      '/ciao.time_tracking.project.ProjectService/Get',
      ($10.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $10.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$10.ListRequest, $10.ListResponse>(
      '/ciao.time_tracking.project.ProjectService/List',
      ($10.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $10.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$10.CreateRequest, $10.CreateResponse>(
      '/ciao.time_tracking.project.ProjectService/Create',
      ($10.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $10.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$10.UpdateRequest, $10.UpdateResponse>(
      '/ciao.time_tracking.project.ProjectService/Update',
      ($10.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $10.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$10.DeleteRequest, $10.DeleteResponse>(
      '/ciao.time_tracking.project.ProjectService/Delete',
      ($10.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $10.DeleteResponse.fromBuffer(value));

  ProjectServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$10.GetResponse> get($10.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$10.ListResponse> list($10.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$10.CreateResponse> create($10.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$10.UpdateResponse> update($10.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$10.DeleteResponse> delete($10.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.project.ProjectService')
abstract class ProjectServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.project.ProjectService';

  ProjectServiceBase() {
    $addMethod($grpc.ServiceMethod<$10.GetRequest, $10.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $10.GetRequest.fromBuffer(value),
        ($10.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$10.ListRequest, $10.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $10.ListRequest.fromBuffer(value),
        ($10.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$10.CreateRequest, $10.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $10.CreateRequest.fromBuffer(value),
        ($10.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$10.UpdateRequest, $10.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $10.UpdateRequest.fromBuffer(value),
        ($10.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$10.DeleteRequest, $10.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $10.DeleteRequest.fromBuffer(value),
        ($10.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$10.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$10.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$10.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$10.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$10.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$10.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$10.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$10.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$10.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$10.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$10.GetResponse> get($grpc.ServiceCall call, $10.GetRequest request);
  $async.Stream<$10.ListResponse> list($grpc.ServiceCall call, $10.ListRequest request);
  $async.Future<$10.CreateResponse> create($grpc.ServiceCall call, $10.CreateRequest request);
  $async.Future<$10.UpdateResponse> update($grpc.ServiceCall call, $10.UpdateRequest request);
  $async.Future<$10.DeleteResponse> delete($grpc.ServiceCall call, $10.DeleteRequest request);
}
