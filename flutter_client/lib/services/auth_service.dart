import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class AuthService {
  static const _storage = FlutterSecureStorage();

  static Future<void> logout() async {
    await _storage.delete(key: 'auth_token');
  }
}
