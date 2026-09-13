import 'package:fluentx/src/rust/foundations/backdrop.dart';

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
    FluentxNativeBackdrop.extend();
    switch (variant ?? current()) {
      case FluentxBackdropVariant.none:
        FluentxNativeBackdrop.none();
        break;
      case FluentxBackdropVariant.mica:
        FluentxNativeBackdrop.mica();
        break;
      case FluentxBackdropVariant.tabbed:
        FluentxNativeBackdrop.tabbed();
        break;
    }
  }
}
