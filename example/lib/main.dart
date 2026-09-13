import 'package:fluentx/fluentx.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  await Fluentx.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late String str;

  @override
  void initState() {
    super.initState();
    str = greet(name: "Nouvist");
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Native Packages')),
        body: SingleChildScrollView(
          child: Container(
            padding: const .all(10),
            child: Center(child: Text(str)),
          ),
        ),
      ),
    );
  }
}
