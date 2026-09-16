import 'package:fluentx/src/rust/foundations/window.dart';

enum FxBackdropVariant { none, mica, tabbed }

abstract final class FxBackdrop {
  static final _history = <FxBackdropVariant>[];

  static FxBackdropVariant current() {
    return _history.lastOrNull ?? .none;
  }

  static void add(FxBackdropVariant variant) {
    _history.add(variant);
    _apply(variant);
  }

  static void pop() {
    _history.removeLast();
    _apply();
  }

  static void _apply([FxBackdropVariant? variant]) {
    final instance = FxNativeWindow.instance();
    instance.extend();
    switch (variant ?? current()) {
      case .none:
        instance.none();
        break;
      case .mica:
        instance.mica();
        break;
      case .tabbed:
        instance.tabbed();
        break;
    }
  }
}
