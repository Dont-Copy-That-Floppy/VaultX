class Record {
  final String id;
  final String title;
  final String encryptedData;

  Record({required this.id, required this.title, required this.encryptedData});

  factory Record.fromJson(Map<String, dynamic> json) {
    return Record(
      id: json['id'],
      title: json['title'],
      encryptedData: json['encrypted_data'],
    );
  }
}
