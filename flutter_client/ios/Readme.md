
---

## **🔹 Step 3: Generate `GoogleService-Info.plist` (iOS)**
To enable Firebase for **iOS**, follow these steps:

### **1️⃣ Add iOS App in Firebase Console**
1. Go to your **Firebase Project**.
2. Click the **iOS icon**.
3. **Enter iOS Bundle ID** (from `ios/Runner/Info.plist`):
   ```
   com.example.valutx
   ```
4. Click **Register App**.
5. **Download `GoogleService-Info.plist`**.
6. Move it to:
   ```
   ios/Runner/GoogleService-Info.plist
   ```

### **2️⃣ Enable Firebase in iOS**
- Open **`ios/Runner/AppDelegate.swift`** and add:
  ```swift
  import UIKit
  import Flutter
  import Firebase

  @UIApplicationMain
  @objc class AppDelegate: FlutterAppDelegate {
    override func application(
      _ application: UIApplication,
      didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
      FirebaseApp.configure()
      return super.application(application, didFinishLaunchingWithOptions: launchOptions)
    }
  }
  ```
- Run:
  ```sh
  cd ios
  pod install
  ```

---

## **🔹 Step 4: Final Checks & Running the App**
1. **Ensure all dependencies are installed**:
   ```sh
   flutter pub get
   ```
2. **For Android**:
   ```sh
   flutter run -d android
   ```
3. **If errors occur, clean & rebuild**:
   ```sh
   flutter clean
   flutter pub get
   flutter build apk --debug
   ```

---