import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../models/record.dart';
import 'encryption_service.dart';
import 'device_info_service.dart';

class ApiService {
  static const String baseUrl = 'http://localhost:8080';
  static const _storage = FlutterSecureStorage();

  /// **Preapprove a new device (sets a flag on the server)**
  static Future<bool> preapproveNewDevice() async {
    final token = await _storage.read(key: 'auth_token');

    final response = await http.post(
      Uri.parse('$baseUrl/devices/preapprove'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
    );

    return response.statusCode == 200;
  }

  /// **Approve a new device (adds the new device ID to the user's account)**
  static Future<bool> approveNewDevice(String deviceId) async {
    final token = await _storage.read(key: 'auth_token');

    final response = await http.post(
      Uri.parse('$baseUrl/devices/approve'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode({"device_id": deviceId}),
    );

    return response.statusCode == 200;
  }

  static Future<bool> register(String username, String password) async {
    final encryptedPassword = EncryptionService.encrypt(password);

    final response = await http.post(
      Uri.parse('$baseUrl/register'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'username': username, 'password': encryptedPassword}),
    );

    return response.statusCode == 200;
  }

  static Future<bool> login(String username, String password) async {
    final encryptedPassword = EncryptionService.encrypt(password);

    final response = await http.post(
      Uri.parse('$baseUrl/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'username': username, 'password': encryptedPassword}),
    );

    if (response.statusCode == 200) {
      final token = jsonDecode(response.body)['token'];
      await _storage.write(key: 'auth_token', value: token);
      return true;
    }
    return false;
  }

  static Future<bool> saveRecord(String title, String data) async {
    final encryptedData = EncryptionService.encrypt(data);
    final token = await _storage.read(key: 'auth_token');

    final response = await http.post(
      Uri.parse('$baseUrl/records'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token',
      },
      body: jsonEncode({'title': title, 'encrypted_data': encryptedData}),
    );

    return response.statusCode == 200;
  }

  static Future<List<Record>> getRecords() async {
    final token = await _storage.read(key: 'auth_token');
    final deviceId = await DeviceInfoService.getDeviceId();

    // Ensure deviceId is not null
    if (token == null || deviceId == null) {
      print("Error: Missing token or deviceId");
      return [];
    }

    final response = await http.get(
      Uri.parse('$baseUrl/secure/records'),
      headers: {
        'Authorization': 'Bearer $token',
        'Device-ID': deviceId, // Now guaranteed to be non-null
      },
    );

    if (response.statusCode == 200) {
      List<dynamic> jsonList = jsonDecode(response.body);
      return jsonList.map((json) => Record.fromJson(json)).toList();
    } else {
      print("Failed to fetch records. Status Code: ${response.statusCode}");
      return [];
    }
  }

  /// **Helper: Read Token Securely**
  static Future<String?> _getToken() async {
    return await _storage.read(key: 'auth_token');
  }

  /// **Fetch Security Logs**
  static Future<List<String>> getLogs() async {
    try {
      final token = await _getToken();
      if (token == null) return [];

      final response = await http.get(
        Uri.parse('$baseUrl/secure/logs'),
        headers: {'Authorization': 'Bearer $token'},
      );

      if (response.statusCode == 200) {
        List<dynamic> jsonList = jsonDecode(response.body);
        return jsonList.map((log) => log.toString()).toList();
      } else {
        print("Failed to fetch logs. Status: ${response.statusCode}");
        return [];
      }
    } catch (e) {
      print("Error fetching logs: $e");
      return [];
    }
  }
}
