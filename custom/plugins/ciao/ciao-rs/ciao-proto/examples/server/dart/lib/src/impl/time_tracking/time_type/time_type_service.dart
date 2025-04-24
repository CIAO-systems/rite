import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/time_type/service.pbgrpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/time_type/time_type.pb.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class TimeTypeService extends TimeTypeServiceBase {
  final JsonStorage<TimeType> _storage;

  TimeTypeService()
      : _storage = JsonStorage<TimeType>('../demodata/time_type_data.json',
            keyFieldName: 'id', createItemFunc: TimeType.create);

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final timeType = _storage.getItem(request.id);
    if (timeType != null) {
      return GetResponse()..timeType = timeType;
    } else {
      throw GrpcError.notFound('Time type not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    // Yield a ListResponse containing the time types (for now, all of them at once)
    yield ListResponse(timeTypes: _storage.getAllItems().toList());
  }

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.timeType);
    return CreateResponse(timeType: request.timeType);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse();
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.timeType);
    return UpdateResponse(timeType: request.timeType);
  }
}
