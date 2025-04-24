import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/accounts/account.pb.dart';
import 'package:grpc_server/src/generated/ciao/accounts/service.pbgrpc.dart'
    as accounts;
import 'package:grpc_server/src/generated/ciao/common/name.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/clock_record.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/service.pbgrpc.dart'
    as timetracking;
import 'package:grpc_server/src/generated/google/protobuf/duration.pb.dart'
    as g;
import 'package:grpc_server/src/generated/google/protobuf/timestamp.pb.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';
import 'package:intl/intl.dart';

class TimeTrackingService extends timetracking.TimeTrackingServiceBase {
  final JsonStorage<ClockRecord> _storage;

  TimeTrackingService()
      : _storage = JsonStorage<ClockRecord>(
            '../demodata/clock_record_data.json',
            keyFieldName: 'id',
            createItemFunc: ClockRecord.create);

  // For calling into the local Account service
  accounts.AccountServiceClient accountServiceClient =
      accounts.AccountServiceClient(ClientChannel('localhost',
          port: 50051,
          options: const ChannelOptions(
            credentials: ChannelCredentials.insecure(),
          )));

  @override
  Future<timetracking.ClockResponse> clock(
      ServiceCall call, ClockRecord request) async {
    final response = timetracking.ClockResponse();
    var clockTime = getClockTime(request.timestamp.timeUtc);

    String name = await getUserName(request.userId);

    response.message = "Thank you for using our services, $name.\n"
        "  - User       : ${request.userId}\n"
        "  - Time       : ${DateFormat('yyyy-MM-dd HH:mm:ss').format(clockTime)}\n"
        "  - Device     : ${request.deviceId}\n"
        "  - Time-Type  : ${request.timeTypeId}\n"
        "  - Project    : ${request.projectId}\n"
        "  - Cost-Center: ${request.costCenterId}\n"
        "Have a nice day!";

    return response;
  }

  DateTime getClockTime(Timestamp timeUtc) {
    Int64 seconds = timeUtc.seconds;
    int nanos = timeUtc.nanos;

    if (seconds != 0 || nanos != 0) {
      return DateTime.fromMillisecondsSinceEpoch(
        seconds.toInt() * 1000 + nanos ~/ 1000000,
      );
    } else {
      return DateTime.now();
    }
  }

  @override
  Future<timetracking.GetResponse> get(
      ServiceCall call, timetracking.GetRequest request) async {
    final clockRecord = _storage.getItem(request.id);
    if (clockRecord != null) {
      return timetracking.GetResponse()..record = clockRecord;
    } else {
      throw GrpcError.notFound('Clock record not found');
    }
  }

  @override
  Stream<timetracking.ListResponse> list(
      ServiceCall call, timetracking.ListRequest request) async* {
    // Yield a ListResponse containing the clock records (for now, all of them at once)
    yield timetracking.ListResponse(records: _storage.getAllItems().toList());
  }

  Future<String> getUserName(String userId) async {
    try {
      var accountRequest = accounts.GetRequest(id: userId);
      final accounts.GetResponse accountResponse =
          await accountServiceClient.get(accountRequest);

      if (accountResponse.hasAccount()) {
        Name name = accountResponse.account.name;
        return "${name.first} ${name.last}";
      } else {
        return "$userId not found";
      }
    } catch (e) {
      print("Error fetching user name: $e");
      return "$userId not found";
    }
  }

  @override
  Future<timetracking.UserInfoResponse> getUserInfo(
      ServiceCall call, timetracking.UserInfoRequest request) async {
    final userId = request.hasUserId() ? request.userId : "unknown";

    // Dummy durations
    final entitlement = g.Duration()..seconds = Int64(30 * 86400); // 30 days
    final remaining = g.Duration()..seconds = Int64(25 * 86400); // 25 days left
    final planned = g.Duration()..seconds = Int64(5 * 86400); // 5 days planned
    final balance = g.Duration()..seconds = Int64(8 * 3600); // 8 hours

    // Fallback account in case fetch fails
    Account account = Account()
      ..id = userId
      ..name = Name(first: "Jane", last: "Doe");

    // Attempt to fetch real account from your AccountService
    try {
      final accountRequest = accounts.GetRequest(id: userId);
      final accountResponse = await accountServiceClient.get(accountRequest);
      if (accountResponse.hasAccount()) {
        account = accountResponse.account;
      }
    } catch (e) {
      print(
          "getUserInfo: Could not load user $userId, using fallback. Error: $e");
    }

    return timetracking.UserInfoResponse()
      ..account = account
      ..balance = timetracking.BalanceInfo(current: balance)
      ..vacation = timetracking.VacationInfo(
        entitlement: entitlement,
        remaining: remaining,
        planned: planned,
      );
  }

  @override
  Future<timetracking.UpdateResponse> update(
      ServiceCall call, timetracking.UpdateRequest request) async {
    _storage.addItem(request.record);
    return timetracking.UpdateResponse(record: request.record);
  }
}
