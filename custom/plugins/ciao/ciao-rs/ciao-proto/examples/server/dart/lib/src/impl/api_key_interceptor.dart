import 'package:grpc/grpc.dart';

// ignore: constant_identifier_names
const String API_KEY = "x-api-key";

Future<GrpcError?> apiKeyInterceptor(
    ServiceCall call, ServiceMethod method) async {
  if (call.clientMetadata != null) {
    if (call.clientMetadata!.containsKey(API_KEY)) {
      String? apiKey = call.clientMetadata![API_KEY];
      if (apiKey != null) {
        print('$API_KEY: $apiKey');
        if (apiKey == 'top-secret-api-key') {
          // If the API key is valid, the call can continue
          return null;
        } else {
          throw Exception("Invalid API Key provided");        
        }
      }
    }
  }

  throw Exception("No API Key provided");
}
