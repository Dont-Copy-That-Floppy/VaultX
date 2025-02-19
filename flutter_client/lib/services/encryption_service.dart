import 'dart:convert';
import 'dart:typed_data';
import 'dart:math';
import 'package:encrypt/encrypt.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class EncryptionService {
  static final _random = Random.secure();
  static final _storage = FlutterSecureStorage();

  /// **Generate a Secure Random 32-Byte Key**
  static Uint8List _generateRandomKey() {
    return Uint8List.fromList(
      List<int>.generate(32, (_) => _random.nextInt(256)),
    );
  }

  /// **Retrieve or Generate AES Key Securely**
  static Future<Key> getKey() async {
    String? storedKey = await _storage.read(key: 'aes_key');

    if (storedKey == null) {
      // Generate and store new key
      Uint8List newKey = _generateRandomKey();
      await _storage.write(key: 'aes_key', value: base64Encode(newKey));
      return Key(newKey);
    }

    return Key(base64Decode(storedKey));
  }

  /// **Generate a Random 16-Byte IV**
  static IV generateIV() {
    final List<int> randomBytes = List<int>.generate(
      16,
      (_) => _random.nextInt(256),
    );
    return IV(Uint8List.fromList(randomBytes));
  }

  /// **Encrypt Data with AES-256-CBC**
  static Future<String> encrypt(String data) async {
    final key = await getKey(); // Retrieve secure key
    final iv = generateIV();
    final encrypter = Encrypter(AES(key, mode: AESMode.cbc));

    final encrypted = encrypter.encrypt(data, iv: iv);
    return base64Encode(iv.bytes + encrypted.bytes); // Store IV + Ciphertext
  }

  /// **Decrypt AES-256-CBC Encrypted Data**
  static Future<String> decrypt(String encryptedData) async {
    try {
      await Future.delayed(
        const Duration(milliseconds: 300),
      ); // Simulate decryption delay

      final key = await getKey(); // Retrieve secure key
      final decodedBytes = base64Decode(encryptedData);
      final iv = IV(
        Uint8List.fromList(decodedBytes.sublist(0, 16)),
      ); // Extract IV
      final encryptedBytes = Uint8List.fromList(
        decodedBytes.sublist(16),
      ); // Extract Ciphertext

      final encrypter = Encrypter(AES(key, mode: AESMode.cbc));
      final decrypted = encrypter.decrypt(Encrypted(encryptedBytes), iv: iv);

      return decrypted;
    } catch (e) {
      return "Decryption failed: $e";
    }
  }
}
