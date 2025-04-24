import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/project/task/project_task.pb.dart';
import 'package:grpc_server/src/generated/ciao/time_tracking/project/task/service.pbgrpc.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class ProjectTaskService extends ProjectTaskServiceBase {
  final JsonStorage<ProjectTask> _storage;

  ProjectTaskService()
      : _storage = JsonStorage<ProjectTask>(
            '../demodata/project_tasks_data.json',
            keyFieldName: 'id',
            createItemFunc: ProjectTask.create);

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    _storage.addItem(request.task);
    return CreateResponse(task: request.task);
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _storage.removeItem(request.id);
    return DeleteResponse();
  }

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final projectTask = _storage.getItem(request.id);
    if (projectTask != null) {
      return GetResponse()..task = projectTask;
    } else {
      throw GrpcError.notFound('Project task not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    // Yield a ListResponse containing the project tasks (for now, all of them at once)
    yield ListResponse(tasks: _storage.getAllItems().toList());
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _storage.addItem(request.task);
    return UpdateResponse(task: request.task);
  }
}
