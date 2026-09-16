import 'package:fluentx/src/rust/foundations/window.dart';

enum FluentxBackdropVariant { none, mica, tabbed }

abstract final class FluentxBackdrop {
  static final _history = <FluentxBackdropVariant>[];

  static FluentxBackdropVariant current() {
    return _history.lastOrNull ?? .none;
  }

  static void add(FluentxBackdropVariant variant) {
    _history.add(variant);
    _apply(variant);
  }

  static void pop() {
    _history.removeLast();
    _apply();
  }

  static void _apply([FluentxBackdropVariant? variant]) {
    FluentxNativeWindow.extend();
    switch (variant ?? current()) {
      case FluentxBackdropVariant.none:
        FluentxNativeWindow.none();
        break;
      case FluentxBackdropVariant.mica:
        FluentxNativeWindow.mica();
        break;
      case FluentxBackdropVariant.tabbed:
        FluentxNativeWindow.tabbed();
        break;
    }
  }
}
