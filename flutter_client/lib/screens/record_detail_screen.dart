import 'package:flutter/material.dart';
import '../models/record.dart';
import '../services/encryption_service.dart';

class RecordDetailScreen extends StatelessWidget {
  const RecordDetailScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // Safely extract arguments from ModalRoute
    final Object? args = ModalRoute.of(context)?.settings.arguments;
    if (args == null || args is! Record) {
      return Scaffold(
        appBar: AppBar(title: const Text("Record Details")),
        body: const Center(child: Text("Error: No record data found.")),
      );
    }

    final Record record = args;

    return Scaffold(
      appBar: AppBar(title: Text(record.title)),
      body: FutureBuilder<String>(
        future: EncryptionService.decrypt(
          record.encryptedData,
        ), // Properly decrypt the data
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError || snapshot.data == null) {
            return const Center(
              child: Text(
                "Error decrypting data.",
                style: TextStyle(color: Colors.red),
              ),
            );
          }

          return Padding(
            padding: const EdgeInsets.all(16.0),
            child: SelectableText(snapshot.data ?? "Decryption failed."),
          );
        },
      ),
    );
  }
}
