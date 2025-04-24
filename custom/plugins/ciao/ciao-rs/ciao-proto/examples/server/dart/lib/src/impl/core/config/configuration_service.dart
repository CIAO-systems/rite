import 'dart:io';
import 'dart:convert';
import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/core/config/service.pbgrpc.dart';

class ConfigurationService extends ConfigurationServiceBase {
  late Map<String, String> configData;

  ConfigurationService() {
    // Load configuration data from JSON file
    final file = File('../demodata/config_data.json');
    final jsonString = file.readAsStringSync();
    configData = Map<String, String>.from(json.decode(jsonString));
  }

  @override
  Future<ConfigurationValuesResponse> getConfigurationValues(
      ServiceCall call, ConfigurationValuesRequest request) async {
    final response = ConfigurationValuesResponse();

    if (request.keys.isEmpty) {
      // If no keys are specified, return all configuration values
      response.entries.addAll(configData);
    } else {
      // Return only the requested keys
      for (final key in request.keys) {
        if (configData.containsKey(key)) {
          response.entries[key] = configData[key]!;
        }
      }
    }

    return response;
  }

  @override
  Future<ServerInfoResponse> getServerInfo(
      ServiceCall call, ServerInfoRequest request) async {
    return ServerInfoResponse(
        version: "1.0.0", buildTime: "2025-01-01T00:00:00Z");
  }
}
