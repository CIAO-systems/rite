import 'dart:convert';
import 'dart:io';

import 'package:grpc_server/src/generated/ciao/core/auth/service.pbgrpc.dart';

class AccountEmailMapping {
  final String accountId;
  final String email;

  AccountEmailMapping(this.accountId, this.email);

  factory AccountEmailMapping.fromJson(Map<String, dynamic> json) {
    return AccountEmailMapping(
        json['accountId'] as String, json['email'] as String);
  }

  @override
  String toString() {
    return "accountId: $accountId, email: $email";
  }
}

class LoginResultData {
  late List<LoginResult> loginResults = [];
  late List<AccountEmailMapping> accountEmailMapping = [];

  LoginResultData() {
    _loadLoginResultData();
    _loadAccountEmailMapping();
  }

  void _loadLoginResultData() {
    try {
      final fileContent =
          File("../demodata/login-result.json").readAsStringSync();
      final List<dynamic> json = jsonDecode(fileContent);

      loginResults = json
          .map((element) => LoginResult.create()..mergeFromProto3Json(element))
          .toList();
    } catch (e) {
      print("Error loading demo result data: $e");
    }
  }

  void _loadAccountEmailMapping() {
    try {
      final fileContent = File("../demodata/email-login-result-mapping.json")
          .readAsStringSync();
      final List<dynamic> json = jsonDecode(fileContent);

      accountEmailMapping =
          json.map((element) => AccountEmailMapping.fromJson(element)).toList();
    } catch (e) {
      print("Error loading demo result data: $e");
    }
  }

  LoginResult? findByEmail(String email) {
    try {
      var mapping = accountEmailMapping.firstWhere(
        (mapping) => mapping.email == email,
        orElse: () => AccountEmailMapping('', ''),
      );
      if (mapping.accountId.isEmpty) {
        return null;
      }

      return loginResults.firstWhere((result) {
        return result.account.id == mapping.accountId;
      });
    } on Exception catch (e) {
      print(e);
    }
    return null;
  }
}
