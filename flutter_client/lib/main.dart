import 'package:flutter/material.dart';
import 'package:firebase_core/firebase_core.dart';
import 'screens/login_screen.dart';
import 'services/notification_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Firebase.initializeApp();  // Initialize Firebase
  await NotificationService.initialize(); // Initialize notifications
  runApp(const ValutXApp());
}

class ValutXApp extends StatelessWidget {
  const ValutXApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ValutX',
      theme: ThemeData.dark(),
      home: const LoginScreen(),
    );
  }
}
