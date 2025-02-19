import 'package:flutter/material.dart';
import '../services/api_service.dart';

class DeviceManagementScreen extends StatelessWidget {
  const DeviceManagementScreen({super.key});

  Future<void> _preapproveNewDevice(BuildContext context) async {
    bool success = await ApiService.preapproveNewDevice();
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(success ? "Preapproval enabled" : "Preapproval failed"),
      ),
    );
  }

  Future<void> _approveDevice(BuildContext context, String deviceId) async {
    bool success = await ApiService.approveNewDevice(deviceId);
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(success ? "Device approved" : "Approval failed")),
    );
  }

  @override
  Widget build(BuildContext context) {
    final TextEditingController _deviceIdController = TextEditingController();

    return Scaffold(
      appBar: AppBar(title: const Text('Device Management')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            ElevatedButton(
              onPressed: () => _preapproveNewDevice(context),
              child: const Text('Enable Device Approval'),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _deviceIdController,
              decoration: const InputDecoration(
                labelText: 'Device ID to Approve',
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed:
                  () => _approveDevice(context, _deviceIdController.text),
              child: const Text('Approve Device'),
            ),
          ],
        ),
      ),
    );
  }
}
