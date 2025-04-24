import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/common/date.pb.dart';
import 'package:grpc_server/src/generated/ciao/common/time_range.pb.dart';
import 'package:grpc_server/src/generated/ciao/common/timestamp.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/absences/absence.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/absences/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class AbsenceService extends AbsenceServiceBase {
  final JsonStorage<Absence> _storage;

  AbsenceService()
      : _storage = JsonStorage<Absence>(
          '../demodata/absences_data.json',
          keyFieldName: 'id',
          createItemFunc: Absence.create,
        );

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    final absence = request.absence;
    _storage.addItem(absence);
    return CreateResponse(absence: absence);
  }

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final absence = _storage.getItem(request.id);
    if (absence != null) {
      return GetResponse()..absence = absence;
    } else {
      throw GrpcError.notFound('Absence not found');
    }
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    final updated = request.absence;
    _storage.addItem(updated);
    return UpdateResponse(absence: updated);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse(); // Empty response
  }

  @override
  Stream<ListResponse> list(
    ServiceCall call,
    ListRequest request,
  ) async* {
    final allAbsences = _storage.getAllItems();

    final hasUserIds = request.userIds.isNotEmpty;
    final hasTimeTypeIds = request.timeTypeIds.isNotEmpty;
    final hasTimeRange = request.hasTimeRange();

    final filtered = allAbsences.where((absence) {
      final matchesUser =
          !hasUserIds || request.userIds.contains(absence.userId);
      final matchesTimeType =
          !hasTimeTypeIds || request.timeTypeIds.contains(absence.timeTypeId);
      final matchesTimeRange =
          !hasTimeRange || _isWithinRange(absence, request.timeRange);
      return matchesUser && matchesTimeType && matchesTimeRange;
    }).toList();

    // Yield a single batch for now; you can chunk this if needed
    yield ListResponse()..absences.addAll(filtered);
  }

  bool _isWithinRange(Absence absence, TimeRange range) {
    if (!range.hasStartTime() || !range.hasEndTime()) return true;

    final start = _fromProtoDate(absence.startDate);
    final end = _fromProtoDate(absence.endDate);

    final rangeStart = _fromProtoTimestamp(range.startTime);
    final rangeEnd = _fromProtoTimestamp(range.endTime);

    return start.isBefore(rangeEnd) && end.isAfter(rangeStart);
  }

  DateTime _fromProtoDate(Date date) {
    return DateTime.utc(date.year, date.month, date.day);
  }

  DateTime _fromProtoTimestamp(Timestamp ts) {
    return DateTime.fromMillisecondsSinceEpoch(
        ts.timeUtc.seconds.toInt() * 1000);
  }
}
