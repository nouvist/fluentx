import 'dart:async';

import 'package:fluentx/fluentx.dart';
import 'package:fluentx/src/rust/foundations/window.dart';

class FluentxTheme extends StatefulWidget {
  final Widget child;

  const FluentxTheme({super.key, required this.child});

  static FluentxThemeData? maybeOf(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<_Inherited>()?.data;
  }

  static FluentxThemeData of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<_Inherited>()!.data;
  }

  @override
  State<FluentxTheme> createState() => _FluentxThemeState();
}

class _FluentxThemeState extends State<FluentxTheme>
    implements FluentxThemeData {
  @override
  late FluentxColors colors;

  var _key = UniqueKey();
  var _listener = null as FluentxNativeWindowListener?;
  var _timer = null as Timer?;

  @override
  void initState() {
    super.initState();
    colors = .current();
    yieldNow(() async {
      _listener = await FluentxNativeWindow.instance().listen(
        callback: _handleRefresh,
      );
    });
  }

  @override
  void dispose() {
    super.dispose();
    if (_listener case final it?) {
      FluentxNativeWindow.instance().cancel(listener: it);
    }
  }

  void _handleRefresh() async {
    _timer?.cancel();
    _timer = .new(.new(milliseconds: 100), () {
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
  FluentxColors get colors;
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
