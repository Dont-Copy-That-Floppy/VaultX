# ValutX
A zero trust, self hosted client and server password hosting


## Project Overview

This proposal outlines the implementation of a **secure password management system** with a **Rust backend** and a **Flutter-based mobile application**. The project emphasizes privacy and a multi-layered security approach using **device-specific identifiers**, **password-based encryption**, and **biometric verification**. The system is designed to provide robust user authentication through a **zero-trust model** after an initial setup, ensuring that only authorized devices and users can access the encrypted data.

### Key Features
- **Collection and Record-Level Security**: Collections are encrypted using both a user password and a hardware-specific ID. Each record's detailed information is protected by **FIDO**-based biometric authentication.
- **Zero Trust Model**: After initial setup, all subsequent data access requests are validated against a stored device-hash for maximum security.
- **Expandable Structure**: Supports different collection types, making it scalable for future categories such as Bank Info, Secure Notes, and Documents.

## Project Requirements

### Technologies
- **Server-Side (Rust Backend)**:
  - Language: Rust
  - Frameworks/Libraries: `actix-web`, `mongodb`, `rust-crypto` (for encryption)
  - Database: MongoDB

- **Client-Side (Flutter Mobile App)**:
  - Language: Dart
  - Frameworks/Libraries: Flutter SDK, `http`, `local_auth` (biometric auth), `encrypt` (AES encryption)

### Dependencies
- **Server-Side**:
  - `actix-web` for creating RESTful APIs.
  - `mongodb` crate for interacting with the MongoDB database.
  - `hmac` and `sha2` for secure hash generation.
- **Client-Side**:
  - `flutter_secure_storage` for secure local storage.
  - `encrypt` for client-side AES encryption.
  - `local_auth` for integrating fingerprint and facial authentication.

### System Requirements
- **Backend**: Rust environment, MongoDB, HTTPS endpoint for secure communication.
- **Mobile**: Android or iOS device capable of biometric authentication, Flutter development environment.

## Project Structure

### File Tree

#### Server-Side (Rust Backend)
```plaintext
rust_backend/
│
├── src/
│   ├── main.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── registration.rs
│   │   ├── authentication.rs
│   │   ├── records.rs
│   │   └── devices.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── mongo_client.rs
│   │   └── collections.rs
│   ├── handlers/
│   │   ├── register_handler.rs
│   │   ├── auth_handler.rs
│   │   └── records_handler.rs
│   ├── utils/
│   │   ├── encryption.rs
│   │   ├── hashing.rs
│   │   └── key_management.rs
│   ├── models/
│   │   ├── user.rs
│   │   ├── device.rs
│   │   └── record.rs
│   └── config/
│       ├── mod.rs
│       └── config.rs
│
├── Cargo.toml
└── .env
```

#### Client-Side (Flutter Mobile App)
```plaintext
flutter_client/
│
├── lib/
│   ├── main.dart
│   ├── screens/
│   │   ├── registration_screen.dart
│   │   ├── login_screen.dart
│   │   ├── home_screen.dart
│   │   ├── record_detail_screen.dart
│   ├── models/
│   │   ├── user.dart
│   │   ├── record.dart
│   └── services/
│       ├── api_service.dart
│       ├── auth_service.dart
│       ├── encryption_service.dart
│       └── device_info_service.dart
│
├── pubspec.yaml
├── android/
│   └── ... (Platform-specific code for Android, e.g., permissions)
├── ios/
│   └── ... (Platform-specific code for iOS, e.g., permissions)
└── assets/
    └── ... (Any static assets like images, icons, etc.)
```

## Done List
- **Backend API Setup**: Implemented basic Rust backend using `actix-web` to create RESTful endpoints for device registration, login, and record management.
- **Database Integration**: Integrated **MongoDB** for storing device registration hashes, user information, and encrypted records.
- **Initial Device Registration**: Created a "Trust All" mode to allow initial device registration by accepting both password and hardware ID, which is then hashed and stored for future reference.
- **Encryption and Hashing Utilities**: Implemented key derivation using **HMAC-SHA256** and AES encryption for securing both collections and individual records.
- **Flutter App Basic Screens**: Developed core UI screens in Flutter for registration, login, home, and record viewing.
- **Biometric Authentication**: Implemented **local_auth** for biometric verification (fingerprint or facial recognition) to ensure record-level access security.

## TODO List
- **Frontend and Backend Integration**: Finalize integration between the Flutter app and Rust backend to facilitate API requests for secure data transfer.
- **Complete Device Verification Logic**: Ensure that all subsequent device interactions require a hash generated from both the **user password** and **hardware ID** to enforce the zero-trust model.
- **Enhanced Error Handling**: Implement robust error handling for network failures, incorrect password inputs, and biometric verification failures.
- **Data Encryption on Device**: Finalize the **encryption_service.dart** in Flutter to handle client-side decryption of titles and secure re-encryption after data access.
- **Automated Testing**: Develop automated tests for both client-side (Flutter) and server-side (Rust) components to verify encryption workflows, registration, and authentication logic.
- **Scalability and Performance Testing**: Conduct performance testing, especially on encryption/decryption operations, to ensure the app functions smoothly even with large datasets.

## Verbose Description of the Project

The **Secure Password Manager** project aims to provide users with a highly secure platform for storing sensitive information like passwords, bank details, and notes. Unlike traditional password managers, this project ensures security at two distinct levels. During initial setup, a **Trust All mode** allows devices to be securely registered with the server by using both the **user password** and a **hardware-specific ID**. This generates a **combined hash** that is stored in a MongoDB database, ensuring future requests are validated against this hash.

After the initial setup, a **zero-trust model** is employed for all further access, meaning that every request made by the device to the server must prove its identity using the same **password + hardware ID hash** combination. This method ensures that even if an unauthorized individual gains access to the user's password, they would still require the physical device for successful authentication.

Furthermore, access to specific details of each record is secured with **FIDO-based biometric authentication**. This adds a second layer of security, meaning even registered devices must provide additional user verification to access sensitive details, such as usernames and passwords. The backend is written in **Rust** for its memory safety, performance, and reliability. The frontend is built using **Flutter**, which provides a cross-platform UI ensuring both **Android** and **iOS** devices are supported seamlessly.

This project represents a **cutting-edge approach** to secure password management by utilizing modern encryption techniques, **multi-factor authentication**, and a **zero-trust philosophy**. The structure also facilitates easy expansion, allowing the future addition of collection types like **bank info**, **secure notes**, or **documents** without altering the core security framework.

