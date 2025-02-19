import 'package:flutter/material.dart';
import '../services/api_service.dart';

class RegistrationScreen extends StatefulWidget {
  const RegistrationScreen({super.key});

  @override
  _RegistrationScreenState createState() => _RegistrationScreenState();
}

class _RegistrationScreenState extends State<RegistrationScreen> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  void _register() async {
    final success = await ApiService.register(
      _usernameController.text,
      _passwordController.text,
    );

    if (success) {
      print("Registration successful");
    } else {
      print("Registration failed");
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Register')),
      body: Column(
        children: [
          TextField(controller: _usernameController, decoration: const InputDecoration(labelText: 'Username')),
          TextField(controller: _passwordController, obscureText: true, decoration: const InputDecoration(labelText: 'Password')),
          ElevatedButton(onPressed: _register, child: const Text('Register')),
        ],
      ),
    );
  }
}
