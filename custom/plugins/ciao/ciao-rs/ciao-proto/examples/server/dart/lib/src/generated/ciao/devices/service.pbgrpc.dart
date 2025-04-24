//
//  Generated code. Do not modify.
//  source: ciao/devices/service.proto
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

import 'service.pb.dart' as $3;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.devices.DeviceService')
class DeviceServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$3.GetRequest, $3.GetResponse>(
      '/ciao.devices.DeviceService/Get',
      ($3.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$3.ListRequest, $3.ListResponse>(
      '/ciao.devices.DeviceService/List',
      ($3.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$3.CreateRequest, $3.CreateResponse>(
      '/ciao.devices.DeviceService/Create',
      ($3.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$3.UpdateRequest, $3.UpdateResponse>(
      '/ciao.devices.DeviceService/Update',
      ($3.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$3.DeleteRequest, $3.DeleteResponse>(
      '/ciao.devices.DeviceService/Delete',
      ($3.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.DeleteResponse.fromBuffer(value));
  static final _$getDeviceConfiguration = $grpc.ClientMethod<$3.DeviceConfigurationRequest, $3.DeviceConfigurationResponse>(
      '/ciao.devices.DeviceService/GetDeviceConfiguration',
      ($3.DeviceConfigurationRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $3.DeviceConfigurationResponse.fromBuffer(value));

  DeviceServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$3.GetResponse> get($3.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$3.ListResponse> list($3.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$3.CreateResponse> create($3.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$3.UpdateResponse> update($3.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$3.DeleteResponse> delete($3.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }

  $grpc.ResponseFuture<$3.DeviceConfigurationResponse> getDeviceConfiguration($3.DeviceConfigurationRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getDeviceConfiguration, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.devices.DeviceService')
abstract class DeviceServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.devices.DeviceService';

  DeviceServiceBase() {
    $addMethod($grpc.ServiceMethod<$3.GetRequest, $3.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.GetRequest.fromBuffer(value),
        ($3.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.ListRequest, $3.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $3.ListRequest.fromBuffer(value),
        ($3.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.CreateRequest, $3.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.CreateRequest.fromBuffer(value),
        ($3.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.UpdateRequest, $3.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.UpdateRequest.fromBuffer(value),
        ($3.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.DeleteRequest, $3.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.DeleteRequest.fromBuffer(value),
        ($3.DeleteResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.DeviceConfigurationRequest, $3.DeviceConfigurationResponse>(
        'GetDeviceConfiguration',
        getDeviceConfiguration_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.DeviceConfigurationRequest.fromBuffer(value),
        ($3.DeviceConfigurationResponse value) => value.writeToBuffer()));
  }

  $async.Future<$3.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$3.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$3.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$3.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$3.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$3.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$3.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$3.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$3.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$3.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$3.DeviceConfigurationResponse> getDeviceConfiguration_Pre($grpc.ServiceCall call, $async.Future<$3.DeviceConfigurationRequest> request) async {
    return getDeviceConfiguration(call, await request);
  }

  $async.Future<$3.GetResponse> get($grpc.ServiceCall call, $3.GetRequest request);
  $async.Stream<$3.ListResponse> list($grpc.ServiceCall call, $3.ListRequest request);
  $async.Future<$3.CreateResponse> create($grpc.ServiceCall call, $3.CreateRequest request);
  $async.Future<$3.UpdateResponse> update($grpc.ServiceCall call, $3.UpdateRequest request);
  $async.Future<$3.DeleteResponse> delete($grpc.ServiceCall call, $3.DeleteRequest request);
  $async.Future<$3.DeviceConfigurationResponse> getDeviceConfiguration($grpc.ServiceCall call, $3.DeviceConfigurationRequest request);
}
