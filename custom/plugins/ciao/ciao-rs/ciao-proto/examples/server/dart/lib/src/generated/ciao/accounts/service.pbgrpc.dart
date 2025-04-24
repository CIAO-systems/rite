//
//  Generated code. Do not modify.
//  source: ciao/accounts/service.proto
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

import 'service.pb.dart' as $0;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.accounts.AccountService')
class AccountServiceClient extends $grpc.Client {
  static final _$get = $grpc.ClientMethod<$0.GetRequest, $0.GetResponse>(
      '/ciao.accounts.AccountService/Get',
      ($0.GetRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.GetResponse.fromBuffer(value));
  static final _$list = $grpc.ClientMethod<$0.ListRequest, $0.ListResponse>(
      '/ciao.accounts.AccountService/List',
      ($0.ListRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.ListResponse.fromBuffer(value));
  static final _$create = $grpc.ClientMethod<$0.CreateRequest, $0.CreateResponse>(
      '/ciao.accounts.AccountService/Create',
      ($0.CreateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.CreateResponse.fromBuffer(value));
  static final _$update = $grpc.ClientMethod<$0.UpdateRequest, $0.UpdateResponse>(
      '/ciao.accounts.AccountService/Update',
      ($0.UpdateRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.UpdateResponse.fromBuffer(value));
  static final _$delete = $grpc.ClientMethod<$0.DeleteRequest, $0.DeleteResponse>(
      '/ciao.accounts.AccountService/Delete',
      ($0.DeleteRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.DeleteResponse.fromBuffer(value));
  static final _$getPersonalMobileActions = $grpc.ClientMethod<$0.PersonalMobileActionsRequest, $0.PersonalMobileActionsResponse>(
      '/ciao.accounts.AccountService/GetPersonalMobileActions',
      ($0.PersonalMobileActionsRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.PersonalMobileActionsResponse.fromBuffer(value));
  static final _$getVersion = $grpc.ClientMethod<$0.GetVersionRequest, $0.GetVersionResponse>(
      '/ciao.accounts.AccountService/GetVersion',
      ($0.GetVersionRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.GetVersionResponse.fromBuffer(value));
  static final _$listVersions = $grpc.ClientMethod<$0.ListVersionsRequest, $0.ListVersionsResponse>(
      '/ciao.accounts.AccountService/ListVersions',
      ($0.ListVersionsRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.ListVersionsResponse.fromBuffer(value));
  static final _$createVersion = $grpc.ClientMethod<$0.CreateVersionRequest, $0.CreateVersionResponse>(
      '/ciao.accounts.AccountService/CreateVersion',
      ($0.CreateVersionRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.CreateVersionResponse.fromBuffer(value));
  static final _$updateVersion = $grpc.ClientMethod<$0.UpdateVersionRequest, $0.UpdateVersionResponse>(
      '/ciao.accounts.AccountService/UpdateVersion',
      ($0.UpdateVersionRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.UpdateVersionResponse.fromBuffer(value));
  static final _$deleteVersion = $grpc.ClientMethod<$0.DeleteVersionRequest, $0.DeleteVersionResponse>(
      '/ciao.accounts.AccountService/DeleteVersion',
      ($0.DeleteVersionRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.DeleteVersionResponse.fromBuffer(value));

  AccountServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$0.GetResponse> get($0.GetRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$get, request, options: options);
  }

  $grpc.ResponseStream<$0.ListResponse> list($0.ListRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$list, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$0.CreateResponse> create($0.CreateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$create, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateResponse> update($0.UpdateRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$update, request, options: options);
  }

  $grpc.ResponseFuture<$0.DeleteResponse> delete($0.DeleteRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$delete, request, options: options);
  }

  $grpc.ResponseFuture<$0.PersonalMobileActionsResponse> getPersonalMobileActions($0.PersonalMobileActionsRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getPersonalMobileActions, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetVersionResponse> getVersion($0.GetVersionRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getVersion, request, options: options);
  }

  $grpc.ResponseStream<$0.ListVersionsResponse> listVersions($0.ListVersionsRequest request, {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$listVersions, $async.Stream.fromIterable([request]), options: options);
  }

  $grpc.ResponseFuture<$0.CreateVersionResponse> createVersion($0.CreateVersionRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createVersion, request, options: options);
  }

  $grpc.ResponseFuture<$0.UpdateVersionResponse> updateVersion($0.UpdateVersionRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$updateVersion, request, options: options);
  }

  $grpc.ResponseFuture<$0.DeleteVersionResponse> deleteVersion($0.DeleteVersionRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$deleteVersion, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.accounts.AccountService')
abstract class AccountServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.accounts.AccountService';

  AccountServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetRequest, $0.GetResponse>(
        'Get',
        get_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetRequest.fromBuffer(value),
        ($0.GetResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListRequest, $0.ListResponse>(
        'List',
        list_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.ListRequest.fromBuffer(value),
        ($0.ListResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateRequest, $0.CreateResponse>(
        'Create',
        create_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CreateRequest.fromBuffer(value),
        ($0.CreateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateRequest, $0.UpdateResponse>(
        'Update',
        update_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.UpdateRequest.fromBuffer(value),
        ($0.UpdateResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteRequest, $0.DeleteResponse>(
        'Delete',
        delete_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.DeleteRequest.fromBuffer(value),
        ($0.DeleteResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PersonalMobileActionsRequest, $0.PersonalMobileActionsResponse>(
        'GetPersonalMobileActions',
        getPersonalMobileActions_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.PersonalMobileActionsRequest.fromBuffer(value),
        ($0.PersonalMobileActionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetVersionRequest, $0.GetVersionResponse>(
        'GetVersion',
        getVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetVersionRequest.fromBuffer(value),
        ($0.GetVersionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListVersionsRequest, $0.ListVersionsResponse>(
        'ListVersions',
        listVersions_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.ListVersionsRequest.fromBuffer(value),
        ($0.ListVersionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateVersionRequest, $0.CreateVersionResponse>(
        'CreateVersion',
        createVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CreateVersionRequest.fromBuffer(value),
        ($0.CreateVersionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UpdateVersionRequest, $0.UpdateVersionResponse>(
        'UpdateVersion',
        updateVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.UpdateVersionRequest.fromBuffer(value),
        ($0.UpdateVersionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeleteVersionRequest, $0.DeleteVersionResponse>(
        'DeleteVersion',
        deleteVersion_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.DeleteVersionRequest.fromBuffer(value),
        ($0.DeleteVersionResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.GetResponse> get_Pre($grpc.ServiceCall call, $async.Future<$0.GetRequest> request) async {
    return get(call, await request);
  }

  $async.Stream<$0.ListResponse> list_Pre($grpc.ServiceCall call, $async.Future<$0.ListRequest> request) async* {
    yield* list(call, await request);
  }

  $async.Future<$0.CreateResponse> create_Pre($grpc.ServiceCall call, $async.Future<$0.CreateRequest> request) async {
    return create(call, await request);
  }

  $async.Future<$0.UpdateResponse> update_Pre($grpc.ServiceCall call, $async.Future<$0.UpdateRequest> request) async {
    return update(call, await request);
  }

  $async.Future<$0.DeleteResponse> delete_Pre($grpc.ServiceCall call, $async.Future<$0.DeleteRequest> request) async {
    return delete(call, await request);
  }

  $async.Future<$0.PersonalMobileActionsResponse> getPersonalMobileActions_Pre($grpc.ServiceCall call, $async.Future<$0.PersonalMobileActionsRequest> request) async {
    return getPersonalMobileActions(call, await request);
  }

  $async.Future<$0.GetVersionResponse> getVersion_Pre($grpc.ServiceCall call, $async.Future<$0.GetVersionRequest> request) async {
    return getVersion(call, await request);
  }

  $async.Stream<$0.ListVersionsResponse> listVersions_Pre($grpc.ServiceCall call, $async.Future<$0.ListVersionsRequest> request) async* {
    yield* listVersions(call, await request);
  }

  $async.Future<$0.CreateVersionResponse> createVersion_Pre($grpc.ServiceCall call, $async.Future<$0.CreateVersionRequest> request) async {
    return createVersion(call, await request);
  }

  $async.Future<$0.UpdateVersionResponse> updateVersion_Pre($grpc.ServiceCall call, $async.Future<$0.UpdateVersionRequest> request) async {
    return updateVersion(call, await request);
  }

  $async.Future<$0.DeleteVersionResponse> deleteVersion_Pre($grpc.ServiceCall call, $async.Future<$0.DeleteVersionRequest> request) async {
    return deleteVersion(call, await request);
  }

  $async.Future<$0.GetResponse> get($grpc.ServiceCall call, $0.GetRequest request);
  $async.Stream<$0.ListResponse> list($grpc.ServiceCall call, $0.ListRequest request);
  $async.Future<$0.CreateResponse> create($grpc.ServiceCall call, $0.CreateRequest request);
  $async.Future<$0.UpdateResponse> update($grpc.ServiceCall call, $0.UpdateRequest request);
  $async.Future<$0.DeleteResponse> delete($grpc.ServiceCall call, $0.DeleteRequest request);
  $async.Future<$0.PersonalMobileActionsResponse> getPersonalMobileActions($grpc.ServiceCall call, $0.PersonalMobileActionsRequest request);
  $async.Future<$0.GetVersionResponse> getVersion($grpc.ServiceCall call, $0.GetVersionRequest request);
  $async.Stream<$0.ListVersionsResponse> listVersions($grpc.ServiceCall call, $0.ListVersionsRequest request);
  $async.Future<$0.CreateVersionResponse> createVersion($grpc.ServiceCall call, $0.CreateVersionRequest request);
  $async.Future<$0.UpdateVersionResponse> updateVersion($grpc.ServiceCall call, $0.UpdateVersionRequest request);
  $async.Future<$0.DeleteVersionResponse> deleteVersion($grpc.ServiceCall call, $0.DeleteVersionRequest request);
}
