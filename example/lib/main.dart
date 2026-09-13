import 'package:fluentx/fluentx.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  await Fluentx.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final accent = FluentxAccentColors.current();
    print(accent.light3.toARGB32().toRadixString(16));
    print(accent.light2.toARGB32().toRadixString(16));
    print(accent.light1.toARGB32().toRadixString(16));
    print(accent.base.toARGB32().toRadixString(16));
    print(accent.dark1.toARGB32().toRadixString(16));
    print(accent.dark2.toARGB32().toRadixString(16));
    print(accent.dark3.toARGB32().toRadixString(16));

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
