
---

## **🔹 Step 2: Generate `google-services.json` (Android)**
To enable Firebase for **Android**, follow these steps:

### **1️⃣ Set Up Firebase in Google Console**
1. Go to **[Firebase Console](https://console.firebase.google.com/)**
2. Click **"Add Project"** → Enter **"ValutX"** and click **Continue**.
3. Disable **Google Analytics** (optional) → Click **Create Project**.
4. After the project is created, click **Continue**.

### **2️⃣ Add Android App**
1. Click the **Android icon** (Add Firebase to your Android app).
2. **Enter package name** (from `android/app/src/main/AndroidManifest.xml`):
   ```
   com.example.valutx
   ```
3. Click **Register App**.
4. **Download `google-services.json`**.
5. Move it to:
   ```
   android/app/google-services.json
   ```

### **3️⃣ Enable Firebase Services in `android/build.gradle`**
- Open **`android/build.gradle`** and add this in `dependencies`:
  ```gradle
  classpath 'com.google.gms:google-services:4.3.10' // Ensure latest version
  ```
- Open **`android/app/build.gradle`** and **apply Google Services plugin**:
  ```gradle
  apply plugin: 'com.google.gms.google-services'
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