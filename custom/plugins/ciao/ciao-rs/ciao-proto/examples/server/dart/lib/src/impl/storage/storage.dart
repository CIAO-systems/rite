import 'dart:convert';
import 'dart:io';

import 'package:protobuf/protobuf.dart';

// Generic JSON Storage Class
class JsonStorage<T extends GeneratedMessage> {
  late Map<String, T> data = {};
  final String filePath;
  String keyFieldName;
  final T Function() createItemFunc; // Store the create function

  JsonStorage(this.filePath,
      {required this.keyFieldName, required this.createItemFunc}) {
    _loadData();
  }

  void _loadData() {
    final file = File(filePath);
    if (!file.existsSync()) {
      // Handle the case where the file doesn't exist, e.g., create it or throw an exception.
      // For example, create an empty file:
      file.createSync();
      return; // or throw Exception('File not found: $filePath');
    }

    final jsonString = file.readAsStringSync();
    if (jsonString.isEmpty) return; //Handle empty files.

    try {
      final List<dynamic> json = jsonDecode(jsonString);
      for (var element in json) {
        T item = createItemFunc()
          ..mergeFromProto3Json(element); // Use a create method
        data[_getKeyFieldValue(item)] = item; // Use a method to get the ID
      }
    } catch (e) {
      // Handle JSON parsing errors, e.g., log the error and continue with an empty map
      print('Error loading JSON: $e, File: $filePath');
      data = {}; // Or rethrow the exception if you want to stop execution.
    }
  }

  void save() {
    // Add a save method
    final jsonList = data.values.map((item) => item.toProto3Json()).toList();
    final jsonString = jsonEncode(jsonList);
    final file = File(filePath);
    file.writeAsStringSync(jsonString);
  }

  T? getItem(String id) => data[id];

  List<T> getAllItems() => data.values.toList();

  void addItem(T item) {
    String key = _getKeyFieldValue(item);
    data[key] = item;
    save();
  }

  void removeItem(String id) {
    data.remove(id);
    save();
  }

  String _getKeyFieldValue(T item) {
    int? tagNumber = item.getTagNumber(keyFieldName);
    return item.getField(tagNumber!);
  }

  T? getItemByField(String fieldName, String value) {
    for (var item in data.values) {
      int? tagNumber = item.getTagNumber(fieldName);
      if (tagNumber != null && item.getField(tagNumber) == value) {
        return item;
      }
    }
    return null;
  }
}
