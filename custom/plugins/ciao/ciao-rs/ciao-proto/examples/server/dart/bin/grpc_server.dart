import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/impl/accounts/account_service.dart';
import 'package:grpc_server/src/impl/api_key_interceptor.dart';
import 'package:grpc_server/src/impl/core/auth/authentication_service.dart';
import 'package:grpc_server/src/impl/core/config/configuration_service.dart';
import 'package:grpc_server/src/impl/devices/devices_service.dart';
import 'package:grpc_server/src/impl/time_tracking/absence/absence_service.dart';
import 'package:grpc_server/src/impl/time_tracking/cost_center/cost_center_service.dart';
import 'package:grpc_server/src/impl/time_tracking/project/project_service.dart';
import 'package:grpc_server/src/impl/time_tracking/project/task/project_task_service.dart';
import 'package:grpc_server/src/impl/time_tracking/time_tracking_service.dart';
import 'package:grpc_server/src/impl/time_tracking/time_type/group/time_type_group_service.dart';
import 'package:grpc_server/src/impl/time_tracking/time_type/time_type_service.dart';

Future<void> main(List<String> arguments) async {
  final server = Server.create(
      services: [
        AbsenceService(),
        AccountService(),
        AuthenticationService(),
        ConfigurationService(),
        CostCenterService(),
        DevicesService(),
        ProjectService(),
        ProjectTaskService(),
        TimeTrackingService(),
        TimeTypeService(),
        TimeTypeGroupService()
      ],
      interceptors: <Interceptor>[
        apiKeyInterceptor
      ],
      codecRegistry:
          CodecRegistry(codecs: const [GzipCodec(), IdentityCodec()]));

  await server.serve(port: 50051);

  // ignore: avoid_print
  print('Server listening on localhost:${server.port}...');
}
