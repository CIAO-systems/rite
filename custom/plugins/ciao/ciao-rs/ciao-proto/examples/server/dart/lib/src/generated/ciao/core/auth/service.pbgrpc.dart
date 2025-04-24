//
//  Generated code. Do not modify.
//  source: ciao/core/auth/service.proto
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

import 'service.pb.dart' as $1;

export 'service.pb.dart';

@$pb.GrpcServiceName('ciao.core.auth.AuthenticationService')
class AuthenticationServiceClient extends $grpc.Client {
  static final _$login = $grpc.ClientMethod<$1.LoginRequest, $1.LoginResult>(
      '/ciao.core.auth.AuthenticationService/Login',
      ($1.LoginRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.LoginResult.fromBuffer(value));

  AuthenticationServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$1.LoginResult> login($1.LoginRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$login, request, options: options);
  }
}

@$pb.GrpcServiceName('ciao.core.auth.AuthenticationService')
abstract class AuthenticationServiceBase extends $grpc.Service {
  $core.String get $name => 'ciao.core.auth.AuthenticationService';

  AuthenticationServiceBase() {
    $addMethod($grpc.ServiceMethod<$1.LoginRequest, $1.LoginResult>(
        'Login',
        login_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.LoginRequest.fromBuffer(value),
        ($1.LoginResult value) => value.writeToBuffer()));
  }

  $async.Future<$1.LoginResult> login_Pre($grpc.ServiceCall call, $async.Future<$1.LoginRequest> request) async {
    return login(call, await request);
  }

  $async.Future<$1.LoginResult> login($grpc.ServiceCall call, $1.LoginRequest request);
}
