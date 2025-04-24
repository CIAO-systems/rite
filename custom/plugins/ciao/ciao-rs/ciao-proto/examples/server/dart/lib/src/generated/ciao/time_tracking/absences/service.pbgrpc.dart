//
//  Generated code. Do not modify.
//  source: ciao/time_tracking/absences/service.proto
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

import 'service.pb.dart' as $8;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.time_tracking.absences.AbsenceService')
class AbsenceServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$8.GetRequest, $8.GetResponse>(
      '/ciao.time_tracking.absences.AbsenceService/Get',
      ($8.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $8.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$8.ListRequest, $8.ListResponse>(
      '/ciao.time_tracking.absences.AbsenceService/List',
      ($8.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $8.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$8.CreateRequest, $8.CreateResponse>(
      '/ciao.time_tracking.absences.AbsenceService/Create',
      ($8.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $8.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$8.UpdateRequest, $8.UpdateResponse>(
      '/ciao.time_tracking.absences.AbsenceService/Update',
      ($8.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $8.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$8.DeleteRequest, $8.DeleteResponse>(
      '/ciao.time_tracking.absences.AbsenceService/Delete',
      ($8.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $8.DeleteResponse.fromBuffer(value));

  AbsenceServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$8.GetResponse> get($8.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$8.ListResponse> list($8.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$8.CreateResponse> create($8.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$8.UpdateResponse> update($8.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$8.DeleteResponse> delete($8.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.time_tracking.absences.AbsenceService')
abstract class AbsenceServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.time_tracking.absences.AbsenceService';

  AbsenceServiceBase() {
    $addMethod($grpc.ServiceMethod<$8.GetRequest, $8.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $8.GetRequest.fromBuffer(value),
        ($8.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$8.ListRequest, $8.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $8.ListRequest.fromBuffer(value),
        ($8.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$8.CreateRequest, $8.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $8.CreateRequest.fromBuffer(value),
        ($8.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$8.UpdateRequest, $8.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $8.UpdateRequest.fromBuffer(value),
        ($8.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$8.DeleteRequest, $8.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $8.DeleteRequest.fromBuffer(value),
        ($8.DeleteResponse value) => value.writeToBuffer()));
  }

  $async.Future<$8.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$8.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$8.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$8.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$8.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$8.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$8.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$8.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$8.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$8.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$8.GetResponse> get($grpc.ServiceCall call, $8.GetRequest request);
  $async.Stream<$8.ListResponse> list($grpc.ServiceCall call, $8.ListRequest request);
  $async.Future<$8.CreateResponse> create($grpc.ServiceCall call, $8.CreateRequest request);
  $async.Future<$8.UpdateResponse> update($grpc.ServiceCall call, $8.UpdateRequest request);
  $async.Future<$8.DeleteResponse> delete($grpc.ServiceCall call, $8.DeleteRequest request);
}
