import 'dart:async';

import 'package:fluentx/fluentx.dart';
import 'package:fluentx/src/rust/foundations/window.dart';

class FxTheme extends StatefulWidget {
  final Widget child;

  const FxTheme({super.key, required this.child});

  static FluentxThemeData? maybeOf(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<_Inherited>()?.data;
  }

  static FluentxThemeData of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<_Inherited>()!.data;
  }

  @override
  State<FxTheme> createState() => _FxThemeState();
}

class _FxThemeState extends State<FxTheme> implements FluentxThemeData {
  @override
  late FxColors colors;

  late int _listener;
  var _key = UniqueKey();
  var _timer = null as Timer?;

  @override
  void initState() {
    super.initState();
    colors = .current();
    _listener = FxNativeWindow.instance().addListener(callback: _handleRefresh);
  }

  @override
  void dispose() {
    FxNativeWindow.instance().removeListener(id: _listener);
    super.dispose();
  }

  void _handleRefresh(FxNativeWindowEvent event) async {
    if (event != .settings) return;
    _timer?.cancel();
    _timer = .new(.new(milliseconds: 100), () async {
      await yieldNow();
      if (!mounted) return;
      setState(() {
        _key = UniqueKey();
        colors = .current();
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return _Inherited(data: this, update: _key, child: widget.child);
  }
}

abstract class FluentxThemeData {
  FxColors get colors;
}

class _Inherited extends InheritedWidget {
  final Key update;
  final FluentxThemeData data;

  const _Inherited({
    required this.update,
    required this.data,
    required super.child,
  });

  @override
  bool updateShouldNotify(covariant _Inherited oldWidget) {
    return oldWidget.update != update;
  }
}
