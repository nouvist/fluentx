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
    final c = FluentxColors.current();

    return MaterialApp(
      home: Scaffold(
        backgroundColor: Colors.transparent,
        body: ListView(
          padding: .all(32),
          children: [
            ColorTile(
              title: Text("Text"),
              children: [
                ColorPreview(c.foreground.primary),
                ColorPreview(c.foreground.secondary),
                ColorPreview(c.foreground.tertiary),
                ColorPreview(c.foreground.disabled),
              ],
            ),
            ColorTile(
              title: Text("Text Accent"),
              children: [
                ColorPreview(c.foregroundAccent.primary),
                ColorPreview(c.foregroundAccent.secondary),
                ColorPreview(c.foregroundAccent.tertiary),
                ColorPreview(c.foregroundAccent.disabled),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class ColorTile extends StatelessWidget {
  final Widget title;
  final List<Widget> children;

  const ColorTile({super.key, required this.title, required this.children});

  @override
  Widget build(BuildContext context) {
    final c = FluentxColors.current();
    return Row(
      children: [
        Expanded(
          child: DefaultTextStyle.merge(
            style: .new(color: c.foreground.primary),
            child: title,
          ),
        ),
        Expanded(flex: 3, child: Wrap(children: children)),
      ],
    );
  }
}

class ColorPreview extends StatelessWidget {
  final Color color;

  const ColorPreview(this.color, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(width: 48, height: 48, color: color);
  }
}
