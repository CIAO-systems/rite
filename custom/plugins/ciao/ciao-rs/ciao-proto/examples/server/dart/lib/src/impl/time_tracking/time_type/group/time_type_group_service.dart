import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/time_type/group/service.pbgrpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/time_type/group/time_type_group.pb.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class TimeTypeGroupService extends TimeTypeGroupServiceBase {
  final JsonStorage<TimeTypeGroup> _storage;

  TimeTypeGroupService()
      : _storage = JsonStorage<TimeTypeGroup>(
          '../demodata/time_type_group_data.json',
          keyFieldName: 'id',
          createItemFunc: TimeTypeGroup.create,
        );

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final group = _storage.getItem(request.id);
    if (group != null) {
      return GetResponse()..timeTypeGroup = group;
    } else {
      throw GrpcError.notFound('Time type group not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    yield ListResponse(
      timeTypeGroups: _storage.getAllItems().toList(),
    );
  }

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.timeTypeGroup);
    return CreateResponse(timeTypeGroup: request.timeTypeGroup);
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.timeTypeGroup);
    return UpdateResponse(timeTypeGroup: request.timeTypeGroup);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse();
  }
}
