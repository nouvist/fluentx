import 'package:fluentx/fluentx.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await Fluentx.init();
  runApp(FxTheme(child: const MyApp()));
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
    FxBackdrop.add(.mica);
  }

  @override
  void dispose() {
    FxBackdrop.pop();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = FxTheme.of(context).colors;

    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        backgroundColor: Colors.transparent,
        body: ListView(
          children: [
            Align(alignment: .topRight, child: FxTitlebarControl()),
            ColorTile(
              title: Text("Brightness"),
              children: [
                switch (c.brightness) {
                  .dark => Text('Dark Mode'),
                  .light => Text('Light Mode'),
                },
              ],
            ),
            ColorTile(
              title: Text("Accent"),
              children: [
                ColorPreview(c.accent.light3),
                ColorPreview(c.accent.light2),
                ColorPreview(c.accent.light1),
                ColorPreview(c.accent.base),
                ColorPreview(c.accent.dark1),
                ColorPreview(c.accent.dark2),
                ColorPreview(c.accent.dark3),
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
    final c = FxColors.current();
    return DefaultTextStyle.merge(
      style: .new(color: c.foreground.primary.primary),
      child: Row(
        children: [
          Expanded(child: title),
          Expanded(flex: 3, child: Row(children: children)),
        ],
      ),
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
