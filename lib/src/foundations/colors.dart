import 'dart:ui';

import 'package:fluentx/src/rust/foundations/colors.dart';

class FluentxColors {
  final FluentxAccentColors accent;
  final FluentxTextColors text;

  const FluentxColors.light([this.accent = .fallback]) : text = .light;
  const FluentxColors.dark([this.accent = .fallback]) : text = .dark;
}

class FluentxAccentColors {
  final Color light3;
  final Color light2;
  final Color light1;
  final Color base;
  final Color dark1;
  final Color dark2;
  final Color dark3;

  const FluentxAccentColors({
    required this.light3,
    required this.light2,
    required this.light1,
    required this.base,
    required this.dark1,
    required this.dark2,
    required this.dark3,
  });

  factory FluentxAccentColors.current() {
    final native = FluentxNativeColors.current();
    if (native == null) return .fallback;
    return .new(
      light3: Color(native.light3),
      light2: Color(native.light2),
      light1: Color(native.light1),
      base: Color(native.base),
      dark1: Color(native.dark1),
      dark2: Color(native.dark2),
      dark3: Color(native.dark3),
    );
  }

  static const fallback = FluentxAccentColors(
    light3: Color(0xff99ebff),
    light2: Color(0xff4cc2ff),
    light1: Color(0xff0091f8),
    base: Color(0xff0078d4),
    dark1: Color(0xff0067c0),
    dark2: Color(0xff003e92),
    dark3: Color(0xff001a68),
  );
}

class FluentxTextColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxTextColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FluentxTextColors(
    primary: Color(0xe4000000),
    secondary: Color(0x9e000000),
    tertiary: Color(0x72000000),
    disabled: Color(0x5c000000),
  );

  static const light = FluentxTextColors(
    primary: Color(0xffffffff),
    secondary: Color(0xc5ffffff),
    tertiary: Color(0x87ffffff),
    disabled: Color(0x5dffffff),
  );
}
