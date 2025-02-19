import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

class NotificationService {
  static final FirebaseMessaging _firebaseMessaging = FirebaseMessaging.instance;
  static final FlutterLocalNotificationsPlugin _localNotifications =
      FlutterLocalNotificationsPlugin();

  /// **Request Notification Permission**
  static Future<void> requestPermission() async {
    NotificationSettings settings = await _firebaseMessaging.requestPermission(
      alert: true,
      announcement: false,
      badge: true,
      carPlay: false,
      criticalAlert: false,
      provisional: false,
      sound: true,
    );

    if (settings.authorizationStatus == AuthorizationStatus.authorized) {
      print("✅ User granted notification permissions.");
    } else if (settings.authorizationStatus == AuthorizationStatus.provisional) {
      print("⚠️ User granted provisional notification permissions.");
    } else {
      print("❌ User denied notification permissions.");
    }
  }

  /// **Retrieve & Print Firebase Token**
  static Future<void> getToken() async {
    String? token = await _firebaseMessaging.getToken();
    if (token != null) {
      print("🔥 Firebase Token: $token");
    } else {
      print("⚠️ Failed to retrieve Firebase Token.");
    }
  }

  /// **Initialize Firebase Messaging & Local Notifications**
  static Future<void> initialize() async {
    await requestPermission();
    await getToken();
    await initLocalNotifications();
    await setupFirebaseListeners();
  }

  /// **Set up Firebase Notification Listeners**
  static Future<void> setupFirebaseListeners() async {
    // Foreground notifications
    FirebaseMessaging.onMessage.listen((RemoteMessage message) {
      print("📩 New foreground notification: ${message.notification?.title}");
      _showLocalNotification(message);
    });

    // Background notifications (when tapped)
    FirebaseMessaging.onMessageOpenedApp.listen((RemoteMessage message) {
      print("🔄 User tapped on a notification: ${message.notification?.title}");
    });

    // Handle notifications received when the app is closed
    FirebaseMessaging.instance.getInitialMessage().then((RemoteMessage? message) {
      if (message != null) {
        print("🟢 Notification received while app was closed: ${message.notification?.title}");
      }
    });
  }

  /// **Initialize Local Notifications for Foreground Alerts**
  static Future<void> initLocalNotifications() async {
    const AndroidInitializationSettings androidSettings =
        AndroidInitializationSettings('@mipmap/ic_launcher');

    final InitializationSettings settings =
        InitializationSettings(android: androidSettings);

    await _localNotifications.initialize(settings);
  }

  /// **Show a Local Notification When App is in Foreground**
  static Future<void> _showLocalNotification(RemoteMessage message) async {
    const AndroidNotificationDetails androidDetails = AndroidNotificationDetails(
      'default_channel_id',
      'Default Notifications',
      importance: Importance.high,
      priority: Priority.high,
    );

    const NotificationDetails platformDetails =
        NotificationDetails(android: androidDetails);

    await _localNotifications.show(
      0,
      message.notification?.title ?? "New Notification",
      message.notification?.body ?? "You have a new message",
      platformDetails,
    );
  }
}
