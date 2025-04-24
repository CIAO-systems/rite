import 'package:dart_jsonwebtoken/dart_jsonwebtoken.dart';
import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/accounts/account.pb.dart';
import 'package:grpc_server/src/generated/ciao/common/name.pb.dart';
import 'package:grpc_server/src/generated/ciao/core/auth/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/demodata/login_results.dart';
import 'package:grpc_server/src/generated/ciao/common/image.pb.dart'
    as ciao_image;

class AuthenticationService extends AuthenticationServiceBase {
  final LoginResultData _loginResults = LoginResultData();

  @override
  Future<LoginResult> login(ServiceCall call, LoginRequest request) async {
    var result = _loginResults.findByEmail(request.email);
    if (result != null) {
      return LoginResult(
        account: result.account,
        token: _createJWT(result.account.id, result.account.name),
      );
    }

    // Some fixed demo user
    switch (request.email) {
      case "demo.user@ciao-systems.com":
        if (request.secret == "secret") {
          String userId = "number-1-employee";
          Name name = Name(first: "Demo", last: "User");
          return LoginResult(
            account: Account(
              id: userId,
              avatar: ciao_image.Image(id: "$userId-avatar"),
              name: name,
            ),
            token: _createJWT(userId, name),
          );
        } else {
          throw GrpcError.unauthenticated("Don't trust you, ${request.email}");
        }
    }
    throw GrpcError.notFound("Don't know you, ${request.email}");
  }

  String _createJWT(String userId, Name name) {
    return JWT(
      {'name': '${name.first} ${name.last}'},
      issuer: "ciao",
      subject: userId,
    ).sign(SecretKey(''));
  }
}
