import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/devices/device.pb.dart';
import 'package:grpc_server/src/generated/ciao/devices/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class DevicesService extends DeviceServiceBase {
  final JsonStorage<Device> _storage;

  DevicesService()
      : _storage = JsonStorage<Device>('../demodata/devices_data.json',
            keyFieldName: 'id', createItemFunc: Device.create);

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.device);
    return CreateResponse(device: request.device);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse();
  }

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final device = _storage.getItem(request.id);
    if (device != null) {
      return GetResponse()..device = device;
    } else {
      throw GrpcError.notFound('Device not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    // Yield a ListResponse containing the devices (for now, all of them at once)
    yield ListResponse(devices: _storage.getAllItems().toList());
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.device);
    return UpdateResponse(device: request.device);
  }

  @override
  Future<DeviceConfigurationResponse> getDeviceConfiguration(
      ServiceCall call, DeviceConfigurationRequest request) async {
    final device = _storage.getItemByField('externalId', request.externalId);
    if (device != null) {
      return DeviceConfigurationResponse()..device = device;
    } else {
      throw GrpcError.notFound('Device not found');
    }
  }
}
