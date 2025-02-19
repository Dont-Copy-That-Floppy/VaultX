import 'dart:convert';
import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'package:webauthn/webauthn.dart';

class WebAuthnService {
  static final String origin = "http://localhost:8080";

  /// **Register User for WebAuthn (Attestation)**
  static Future<bool> register(String userId) async {
    try {
      // 1. Request a challenge from your server.
      final response = await http.post(
        Uri.parse('$origin/webauthn/register'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'user_id': userId}),
      );
      if (response.statusCode != 200) {
        print(
          "❌ Registration request failed with status: ${response.statusCode}",
        );
        return false;
      }

      // Assume the server returns a Base64-encoded challenge.
      final challengeBase64 = jsonDecode(response.body)['challenge'] as String;
      final Uint8List clientDataHash = WebAPI.base64Decode(challengeBase64);

      // 2. Build the Relying Party (RP) and User entities.
      final rpEntity = RpEntity(
        id: 'localhost', // Replace with your actual RP id/domain
        name: 'Localhost',
      );
      final userEntity = UserEntity(
        id: Uint8List.fromList(utf8.encode(userId)), // User id as bytes
        name: userId,
        displayName: userId,
      );

      // 3. Define the credential type & algorithm pair (for example, ES256 with alg value -7).
      final credTypePair = CredTypePubKeyAlgoPair(
        credType: PublicKeyCredentialType.publicKey,
        pubKeyAlgo: -7,
      );

      // 4. Create MakeCredentialOptions.
      final makeCredentialOptions = MakeCredentialOptions(
        clientDataHash: clientDataHash,
        rpEntity: rpEntity,
        userEntity: userEntity,
        requireResidentKey: false,
        requireUserPresence: true,
        requireUserVerification: true,
        credTypesAndPubKeyAlgs: [credTypePair],
        excludeCredentialDescriptorList: null, // or provide a list if needed
      );

      // 5. Instantiate WebAPI.
      final webAPI = WebAPI();

      // 6. Adapt the options using WebAPI. The method returns a tuple:
      //    (CollectedClientData, MakeCredentialOptions) adapted for the authenticator.
      final makeCredentialResult = await webAPI.createMakeCredentialOptions(
        origin,
        makeCredentialOptions as CreateCredentialOptions,
        true, // assuming same origin with ancestor is true
      );
      final collectedClientData = makeCredentialResult.$1;
      final preparedOptions = makeCredentialResult.$2;

      // 7. Use the Authenticator to create a new credential.
      //    (Assumes an Authenticator class with a method makeCredential.)
      final authenticator = Authenticator(true, false);
      final attestation = await authenticator.makeCredential(preparedOptions);

      // 8. Convert the raw attestation into an attestation response.
      final attestationResponse = await webAPI.createAttestationResponse(
        collectedClientData,
        attestation,
      );

      // 9. Send the attestation response to your server for verification.
      final verificationResponse = await http.post(
        Uri.parse('$origin/webauthn/verify'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(attestationResponse.toJson()),
      );
      if (verificationResponse.statusCode == 200) {
        return true;
      } else {
        print(
          "❌ Server verification failed with status: ${verificationResponse.statusCode}",
        );
        return false;
      }
    } catch (e) {
      print("❌ WebAuthn registration error: $e");
      return false;
    }
  }

  /// **Verify WebAuthn Authentication (Assertion)**
  static Future<bool> verify(String userId) async {
    try {
      // 1. Request assertion options from your server (includes a challenge and allowed credentials).
      final response = await http.post(
        Uri.parse('$origin/webauthn/beginAssertion'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'user_id': userId}),
      );
      if (response.statusCode != 200) {
        print(
          "❌ Assertion options request failed with status: ${response.statusCode}",
        );
        return false;
      }
      // Assume the server returns JSON that can be converted into CredentialRequestOptions.
      final assertionOptionsJson = jsonDecode(response.body);
      final credentialRequestOptions = CredentialRequestOptions.fromJson(
        assertionOptionsJson,
      );

      // 2. Instantiate WebAPI.
      final webAPI = WebAPI();

      // 3. Adapt the assertion options.
      final getAssertionResult = await webAPI.createGetAssertionOptions(
        origin,
        credentialRequestOptions,
        true, // assuming same origin with ancestor is true
      );
      final collectedClientData = getAssertionResult.$1;
      final preparedAssertionOptions = getAssertionResult.$2;

      // 4. Use the Authenticator to get an assertion.
      //    (Assumes an Authenticator class with a method getAssertion.)
      final authenticator = Authenticator(true, false);
      final assertion = await authenticator.getAssertion(
        preparedAssertionOptions,
      );

      // 5. Convert the raw assertion into an assertion response.
      final assertionResponse = await webAPI.createAssertionResponse(
        collectedClientData,
        assertion,
      );

      // 6. Send the assertion response to your server for verification.
      final verificationResponse = await http.post(
        Uri.parse('$origin/webauthn/verify'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(assertionResponse.toJson()),
      );
      if (verificationResponse.statusCode == 200) {
        return true;
      } else {
        print(
          "❌ Server verification failed with status: ${verificationResponse.statusCode}",
        );
        return false;
      }
    } catch (e) {
      print("❌ WebAuthn verification error: $e");
      return false;
    }
  }
}
