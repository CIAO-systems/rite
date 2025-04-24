import 'package:grpc/grpc.dart';
import 'package:grpc_server/src/generated/ciao/accounts/account.pb.dart';
import 'package:grpc_server/src/generated/ciao/accounts/account_version.pb.dart';
import 'package:grpc_server/src/generated/ciao/accounts/service.pbgrpc.dart';
import 'package:grpc_server/src/generated/ciao/devices/device.pb.dart';
import 'package:grpc_server/src/impl/storage/storage.dart';

class AccountService extends AccountServiceBase {
  final JsonStorage<Account> _accountStorage;
  final JsonStorage<AccountVersion> _versionStorage;

  AccountService()
      : _accountStorage = JsonStorage<Account>('../demodata/accounts_data.json',
            keyFieldName: 'id', createItemFunc: Account.create),
        _versionStorage = JsonStorage<AccountVersion>(
            '../demodata/account_versions_data.json',
            keyFieldName: 'id',
            createItemFunc: AccountVersion.create);

  @override
  Future<CreateResponse> create(ServiceCall call, CreateRequest request) async {
    final account = request.account;
    _accountStorage.addItem(account);
    return CreateResponse(account: account);
  }

  @override
  Future<GetResponse> get(ServiceCall call, GetRequest request) async {
    final account = _accountStorage.getItem(request.id);
    if (account != null) {
      return GetResponse()..account = account;
    } else {
      throw GrpcError.notFound('Account not found');
    }
  }

  @override
  Stream<ListResponse> list(ServiceCall call, ListRequest request) async* {
    // Yield a ListResponse containing the accounts (for now, all of them at once)
    yield ListResponse(accounts: _accountStorage.getAllItems().toList());
  }

  @override
  Future<DeleteResponse> delete(ServiceCall call, DeleteRequest request) async {
    _accountStorage.removeItem(request.id);
    return DeleteResponse();
  }

  @override
  Future<UpdateResponse> update(ServiceCall call, UpdateRequest request) async {
    _accountStorage.addItem(request.account);
    return UpdateResponse(account: request.account);
  }

  @override
  Future<PersonalMobileActionsResponse> getPersonalMobileActions(
      ServiceCall call, PersonalMobileActionsRequest request) async {
    return PersonalMobileActionsResponse(actions: [
      DeviceAction(
          id: 42,
          deviceActionId: 42,
          icon: "icon",
          name: "name",
          type: DeviceActionType.CLOCK,
          clock: ClockConfiguration(
              timeTypeId: "123e4567-e89b-12d3-a456-426614174000")),
    ]);
  }

  @override
  Future<CreateVersionResponse> createVersion(
      ServiceCall call, CreateVersionRequest request) async {
    final version = request.version;
    _versionStorage.addItem(version);
    return CreateVersionResponse(version: version);
  }

  @override
  Future<DeleteVersionResponse> deleteVersion(
      ServiceCall call, DeleteVersionRequest request) async {
    _versionStorage.removeItem(request.id);
    return DeleteVersionResponse();
  }

  @override
  Future<GetVersionResponse> getVersion(
      ServiceCall call, GetVersionRequest request) async {
    final version = _versionStorage.getItem(request.id);
    if (version != null) {
      return GetVersionResponse()..version = version;
    } else {
      throw GrpcError.notFound('Account version not found');
    }
  }

  @override
  Stream<ListVersionsResponse> listVersions(
      ServiceCall call, ListVersionsRequest request) async* {
    print("listVersions called...");

    final versions = _versionStorage.getAllItems();
    print("Versions retrieved: ${versions.length}");

    if (versions.isEmpty) {
      print("No versions found.");
    }

    yield ListVersionsResponse(versions: versions);
  }

  @override
  Future<UpdateVersionResponse> updateVersion(
      ServiceCall call, UpdateVersionRequest request) async {
    _versionStorage.addItem(request.version);
    return UpdateVersionResponse(version: request.version);
  }
}
