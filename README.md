# **ValutX**
A zero-trust, self-hosted password manager with end-to-end encryption and multi-factor authentication.

## **Project Overview**
ValutX is a **secure, open-source password manager** with a **Rust backend** and **Flutter frontend (Web, Android, iOS)**. It follows a **zero-trust security model**, ensuring that only **explicitly authorized devices** can access encrypted user data.

The backend is **lightweight** and **simple to deploy**, requiring **minimal setup**. After **first-time configuration**, all user data is **encrypted end-to-end** and stored securely in **MongoDB**.

---

## **Core Security Features**
### **🔹 Zero-Trust Architecture**
- **Strict device verification**: Every device is fingerprinted using **hardware ID, IP address, and TLS signature**.
- **No trust assumptions**: If **ANY fingerprint detail changes**, the server **denies access**.
- **All unknown devices and IPs are rejected** by default.

### **🔹 Strong Authentication & Access Control**
- **Master Password Setup**: First-time configuration requires a **master password** to derive an encryption key.
- **AES-256 Encryption**: All user data is stored **encrypted at rest**.
- **Google Authenticator (TOTP) for Sync**: **Every sync request** requires **TOTP authentication**.
- **FIDO/WebAuthn Biometric Authentication**: After device approval, biometrics can be used instead of passwords.
- **Device Expiry Rule**: Devices inactive for **14+ days** require re-authentication.

### **🔹 Secure Data Management**
- **Encrypted Key Backup & Recovery**: Users can **export an encrypted backup** of their server’s data.
- **Strict Device Approval System**: 
  - **Only preapproved devices** can enable **new device registration**.
  - **New devices must be explicitly approved** before they can access the account.
- **Security Audit Logging**: Every **login attempt, device addition, fingerprint mismatch, or failed authentication** is logged.
- **User-Accessible Logs**: Users can **review security logs** in the app.

---

## **Project Structure**
### **Backend (Rust)**
```plaintext
rust_backend/
│
├── src/
│   ├── main.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── authentication.rs
│   │   ├── registration.rs
│   │   ├── records.rs
│   │   ├── devices.rs
│   │   ├── backup.rs
│   │   ├── webauthn.rs
│   ├── db/
│   │   ├── mongo_client.rs
│   │   ├── collections.rs
│   ├── middleware/
│   │   ├── auth_middleware.rs
│   ├── utils/
│   │   ├── encryption.rs
│   │   ├── hashing.rs
│   │   ├── key_management.rs
│   │   ├── logger.rs
│   ├── models/
│   │   ├── user.rs
│   │   ├── device.rs
│   │   ├── record.rs
│   ├── config/
│   │   ├── config.rs
│   ├── Cargo.toml
│   ├── .env
```

### **Frontend (Flutter)**
```plaintext
flutter_client/
│
├── lib/
│   ├── main.dart
│   ├── screens/
│   │   ├── login_screen.dart
│   │   ├── registration_screen.dart
│   │   ├── home_screen.dart
│   │   ├── record_detail_screen.dart
│   │   ├── device_management_screen.dart
│   │   ├── logs_screen.dart
│   ├── models/
│   │   ├── user.dart
│   │   ├── record.dart
│   ├── services/
│   │   ├── api_service.dart
│   │   ├── auth_service.dart
│   │   ├── encryption_service.dart
│   │   ├── device_info_service.dart
│   │   ├── webauthn_service.dart
│   ├── pubspec.yaml
│   ├── android/
│   ├── ios/
│   ├── web/
│   ├── assets/
```

---

## **Current Implementation Status**
| Feature | Status |
|---------|--------|
| **Zero-Trust Security Model** | ✅ Implemented |
| **Master Password Encryption** | ✅ Implemented |
| **AES-256 Encryption for All Data** | ✅ Implemented |
| **Device Fingerprinting (Hardware ID, IP, TLS Signature)** | ✅ Implemented |
| **Reject All Unknown Devices/IPs** | ✅ Implemented |
| **Google Authenticator (TOTP) for Sync** | ✅ Implemented |
| **FIDO/WebAuthn Biometric Authentication** | ✅ Implemented |
| **Device Expiry Rule (14-Day Limit)** | ✅ Implemented |
| **Encrypted Key Backup & Recovery** | ✅ Implemented |
| **Security Audit Logging (MongoDB Logs)** | ✅ Implemented |
| **User-Accessible Logs (Web/Android/iOS UI)** | ✅ Implemented |
| **Push Notifications for Security Alerts** | ❌ Not Implemented |
| **Self-Destruct Mode (Optional Security Feature)** | ❌ Not Implemented |

---

## **How It Works**
### **🔹 First-Time Setup**
1. The server starts and prompts for a **master password**.
2. The master password **encrypts all data in MongoDB**.
3. A **Google Authenticator-compatible TOTP secret** is generated.
4. The first device registers and is **fingerprinted**.
5. This device is now **trusted**.

### **🔹 Regular Use**
1. The client **authenticates using password + TOTP** on first sync.
2. The **server checks fingerprint data**.
   - If **everything matches**, no password/2FA is required.
   - If **ANYTHING changes**, access is **denied**.
3. **Biometric authentication** can be used instead of passwords for fast access.
4. **All security events are logged**.

### **🔹 Adding a New Device**
1. A **preapproved device** must enable **new device registration**.
2. The new device **requests access**.
3. The **preapproved device must explicitly approve it**.
4. The new device can now **use biometric authentication**.
5. If **the new device is inactive for 2 weeks**, it must be reapproved.

### **🔹 Backup & Recovery**
1. Any **trusted device** can request an **encrypted database backup**.
2. The backup is **fully encrypted** using the master password.
3. **Restoration requires the master password** to decrypt.

---

## **Future Enhancements**
🔹 **Push Notifications for Security Alerts** (Unauthorized logins, fingerprint mismatches)  
🔹 **Self-Destruct Mode** (Optional security feature to wipe all data after too many failed attempts)  

---

## **How to Run**
### **1. Start the Rust Backend**
```sh
cd rust_backend
cargo run
```

### **2. Run the Flutter App**
```sh
cd flutter_client
flutter run
```

### **3. Access the Web App**
Open `http://localhost:8080` in a browser.

---

## **License**
ValutX is **open-source software** licensed under **GPLv3**.
