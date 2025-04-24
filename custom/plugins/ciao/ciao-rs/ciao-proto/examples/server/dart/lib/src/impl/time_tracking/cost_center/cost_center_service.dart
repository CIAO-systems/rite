import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/cost_center/cost_center.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/cost_center/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class CostCenterService extends CostCenterServiceBase {
  final JsonStorage<CostCenter> _storage;

  CostCenterService()
      : _storage = JsonStorage<CostCenter>(
          '../demodata/cost_centers_data.json',
          keyFieldName: 'id',
          createItemFunc: CostCenter.create,
        );

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final costCenter = _storage.getItem(request.id);
    if (costCenter != null) {
      return GetResponse()..costCenter = costCenter;
    } else {
      throw GrpcError.notFound('Cost Center not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    yield ListResponse(costCenters: _storage.getAllItems().toList());
  }

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.costCenter);
    return CreateResponse()..costCenter = request.costCenter;
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.costCenter);
    return UpdateResponse()..costCenter = request.costCenter;
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse()..message = "Cost Center deleted successfully";
  }
}
