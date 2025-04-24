import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/project/project.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/project/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class ProjectService extends ProjectServiceBase {
  final JsonStorage<Project> _storage;

  ProjectService()
      : _storage = JsonStorage<Project>('../demodata/projects_data.json',
            keyFieldName: 'id', createItemFunc: Project.create);

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.project);
    return CreateResponse(project: request.project);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse();
  }

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final project = _storage.getItem(request.id);
    if (project != null) {
      return GetResponse()..project = project;
    } else {
      throw GrpcError.notFound('Project not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    // Yield a ListResponse containing the projects (for now, all of them at once)
    yield ListResponse(projects: _storage.getAllItems().toList());
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.project);
    return UpdateResponse(project: request.project);
  }
}
