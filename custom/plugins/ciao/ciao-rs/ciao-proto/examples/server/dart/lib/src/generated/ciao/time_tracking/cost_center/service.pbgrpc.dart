//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/cost_center/service.proto
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

import 'service.pb.dart' as $9;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.cost_center.CostCenterService')
class CostCenterServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$9.GetRequest, $9.GetResponse>(
      '/ciao.time_tracking.cost_center.CostCenterService/Get',
      ($9.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $9.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$9.ListRequest, $9.ListResponse>(
      '/ciao.time_tracking.cost_center.CostCenterService/List',
      ($9.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $9.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$9.CreateRequest, $9.CreateResponse>(
      '/ciao.time_tracking.cost_center.CostCenterService/Create',
      ($9.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $9.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$9.UpdateRequest, $9.UpdateResponse>(
      '/ciao.time_tracking.cost_center.CostCenterService/Update',
      ($9.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $9.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$9.DeleteRequest, $9.DeleteResponse>(
      '/ciao.time_tracking.cost_center.CostCenterService/Delete',
      ($9.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $9.DeleteResponse.fromBuffer(value));

  CostCenterServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$9.GetResponse> get($9.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$9.ListResponse> list($9.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$9.CreateResponse> create($9.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$9.UpdateResponse> update($9.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$9.DeleteResponse> delete($9.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.cost_center.CostCenterService')
abstract class CostCenterServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.cost_center.CostCenterService';

  CostCenterServiceBase() {
    $addMethod($grpc.ServiceMethod<$9.GetRequest, $9.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $9.GetRequest.fromBuffer(value),
        ($9.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$9.ListRequest, $9.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $9.ListRequest.fromBuffer(value),
        ($9.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$9.CreateRequest, $9.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $9.CreateRequest.fromBuffer(value),
        ($9.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$9.UpdateRequest, $9.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $9.UpdateRequest.fromBuffer(value),
        ($9.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$9.DeleteRequest, $9.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $9.DeleteRequest.fromBuffer(value),
        ($9.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$9.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$9.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$9.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$9.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$9.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$9.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$9.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$9.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$9.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$9.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$9.GetResponse> get($grpc.ServiceCall call, $9.GetRequest request);
  $async.Stream<$9.ListResponse> list($grpc.ServiceCall call, $9.ListRequest request);
  $async.Future<$9.CreateResponse> create($grpc.ServiceCall call, $9.CreateRequest request);
  $async.Future<$9.UpdateResponse> update($grpc.ServiceCall call, $9.UpdateRequest request);
  $async.Future<$9.DeleteResponse> delete($grpc.ServiceCall call, $9.DeleteRequest request);
}
