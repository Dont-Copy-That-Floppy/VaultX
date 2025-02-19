import 'package:flutter/material.dart';
import '../services/api_service.dart';

class LogsScreen extends StatefulWidget {
  const LogsScreen({super.key});

  @override
  _LogsScreenState createState() => _LogsScreenState();
}

class _LogsScreenState extends State<LogsScreen> {
  List<String> logs = [];
  bool isLoading = true;
  String? errorMessage;

  @override
  void initState() {
    super.initState();
    _fetchLogs();
  }

  Future<void> _fetchLogs() async {
    try {
      final fetchedLogs = await ApiService.getLogs();
      setState(() {
        logs = fetchedLogs;
        isLoading = false;
      });
    } catch (error) {
      setState(() {
        errorMessage = "Failed to fetch logs. Please try again.";
        isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Security Logs')),
      body: isLoading
          ? const Center(child: CircularProgressIndicator())
          : errorMessage != null
              ? Center(child: Text(errorMessage!, style: TextStyle(color: Colors.red)))
              : logs.isEmpty
                  ? const Center(child: Text('No logs available.'))
                  : ListView.builder(
                      itemCount: logs.length,
                      itemBuilder: (context, index) {
                        return ListTile(
                          title: Text(logs[index]),
                          leading: const Icon(Icons.security),
                        );
                      },
                    ),
    );
  }
}
