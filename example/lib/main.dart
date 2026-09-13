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
  @override
  void initState() {
    super.initState();
    FluentxBackdrop.add(.mica);
  }

  @override
  void dispose() {
    FluentxBackdrop.pop();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final accent = FluentxAccentColors.current();

    return MaterialApp(
      home: Scaffold(
        backgroundColor: Colors.transparent,
        body: Center(
          child: Row(
            mainAxisAlignment: .center,
            crossAxisAlignment: .center,
            children: [
              Container(
                width: 64,
                height: 64,
                color: accent.dark3,
                child: Center(child: Text('Dark3')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.dark2,
                child: Center(child: Text('Dark2')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.dark1,
                child: Center(child: Text('Dark1')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.base,
                child: Center(child: Text('Base')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.light1,
                child: Center(child: Text('Light1')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.light2,
                child: Center(child: Text('Light2')),
              ),
              Container(
                width: 64,
                height: 64,
                color: accent.light3,
                child: Center(child: Text('Light3')),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
