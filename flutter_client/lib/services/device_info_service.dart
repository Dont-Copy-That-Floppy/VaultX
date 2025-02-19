import 'dart:io';
import 'package:device_info_plus/device_info_plus.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class DeviceInfoService {
  static const _storage = FlutterSecureStorage();

  /// **Save JWT auth token securely**
  static Future<void> saveAuthToken(String token) async {
    try {
      await _storage.write(key: 'auth_token', value: token);
    } catch (e) {
      print("Error saving auth token: $e");
    }
  }

  /// **Retrieve JWT auth token**
  static Future<String?> getAuthToken() async {
    try {
      return await _storage.read(key: 'auth_token');
    } catch (e) {
      print("Error reading auth token: $e");
      return null;
    }
  }

  /// **Delete JWT auth token from secure storage**
  static Future<void> deleteAuthToken() async {
    try {
      await _storage.delete(key: 'auth_token');
    } catch (e) {
      print("Error deleting auth token: $e");
    }
  }

  /// **Retrieve unique device ID based on platform**
  static Future<String?> getDeviceId() async {
    try {
      final DeviceInfoPlugin deviceInfo = DeviceInfoPlugin();

      if (Platform.isAndroid) {
        final androidInfo = await deviceInfo.androidInfo;
        return androidInfo.id;
      } else if (Platform.isIOS) {
        final iosInfo = await deviceInfo.iosInfo;
        return iosInfo.identifierForVendor;
      } else if (Platform.isWindows) {
        final windowsInfo = await deviceInfo.windowsInfo;
        return windowsInfo.deviceId;
      } else {
        print("Unsupported platform for device ID retrieval.");
        return null;
      }
    } catch (e) {
      print("Error retrieving device ID: $e");
      return null;
    }
  }
}
