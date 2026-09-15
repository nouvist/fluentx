part of 'colors.dart';

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

class FluentxForegroundPrimaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxForegroundPrimaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FluentxForegroundPrimaryColors(
    primary: Color(0xffffffff),
    secondary: Color(0xc5ffffff),
    tertiary: Color(0x87ffffff),
    disabled: Color(0x5dffffff),
  );

  static const light = FluentxForegroundPrimaryColors(
    primary: Color(0xe4000000),
    secondary: Color(0x9e000000),
    tertiary: Color(0x72000000),
    disabled: Color(0x5c000000),
  );
}

class FluentxForegroundAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxForegroundAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  factory FluentxForegroundAccentColors.dark(FluentxAccentColors accent) =>
      .new(
        primary: accent.light3,
        secondary: accent.light3,
        tertiary: accent.light2,
        disabled: const Color(0x5dffffff),
      );

  factory FluentxForegroundAccentColors.light(FluentxAccentColors accent) =>
      .new(
        primary: accent.dark2,
        secondary: accent.dark3,
        tertiary: accent.dark1,
        disabled: const Color(0x5c000000),
      );
}

class FluentxForegroundOnAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxForegroundOnAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FluentxForegroundOnAccentColors(
    primary: Color(0xff000000),
    secondary: Color(0x80000000),
    tertiary: Color(0x87000000),
    disabled: Color(0xffffffff),
  );

  static const light = FluentxForegroundOnAccentColors(
    primary: Color(0xffffffff),
    secondary: Color(0xb3ffffff),
    tertiary: Color(0xb3ffffff),
    disabled: Color(0xffffffff),
  );
}

class FluentxControlPrimaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color disabled;
  final Color transparent;
  final Color active;

  const FluentxControlPrimaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.disabled,
    required this.transparent,
    required this.active,
  });

  static const dark = FluentxControlPrimaryColors(
    primary: Color(0x0fffffff),
    secondary: Color(0x15ffffff),
    tertiary: Color(0x0bffffff),
    quarternary: Color(0x0fffffff),
    disabled: Color(0x4df9f9f9),
    transparent: Color(0x00ffffff),
    active: Color(0xb31e1e1e),
  );

  static const light = FluentxControlPrimaryColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x80f9f9f9),
    tertiary: Color(0x4df9f9f9),
    quarternary: Color(0xc2f3f3f3),
    disabled: Color(0x4df9f9f9),
    transparent: Color(0x00ffffff),
    active: Color(0xffffffff),
  );
}

class FluentxControlSecondaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color disabled;

  const FluentxControlSecondaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.disabled,
  });

  static const dark = FluentxControlSecondaryColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x19000000),
    tertiary: Color(0x0bffffff),
    quarternary: Color(0x12ffffff),
    disabled: Color(0x00ffffff),
  );

  static const light = FluentxControlSecondaryColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x06000000),
    tertiary: Color(0x0f000000),
    quarternary: Color(0x18000000),
    disabled: Color(0x00ffffff),
  );
}

class FluentxControlSolidColors {
  final Color primary;

  const FluentxControlSolidColors({required this.primary});

  static const dark = FluentxControlSolidColors(primary: Color(0xff454545));
  static const light = FluentxControlSolidColors(primary: Color(0xffffffff));
}

class FluentxControlStrongColors {
  final Color primary;
  final Color disabled;

  const FluentxControlStrongColors({
    required this.primary,
    required this.disabled,
  });

  static const dark = FluentxControlStrongColors(
    primary: Color(0x8bffffff),
    disabled: Color(0x3fffffff),
  );

  static const light = FluentxControlStrongColors(
    primary: Color(0x72000000),
    disabled: Color(0x51000000),
  );
}

class FluentxControlSubtleColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxControlSubtleColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FluentxControlSubtleColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x0fffffff),
    tertiary: Color(0x0affffff),
    disabled: Color(0x00ffffff),
  );

  static const light = FluentxControlSubtleColors(
    primary: Color(0x00000000),
    secondary: Color(0x09000000),
    tertiary: Color(0x06000000),
    disabled: Color(0x00000000),
  );
}

class FluentxControlOnImageColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FluentxControlOnImageColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FluentxControlOnImageColors(
    primary: Color(0xb31c1c1c),
    secondary: Color(0xff1a1a1a),
    tertiary: Color(0xff131313),
    disabled: Color(0xff1e1e1e),
  );

  static const light = FluentxControlOnImageColors(
    primary: Color(0xc9ffffff),
    secondary: Color(0xfff3f3f3),
    tertiary: Color(0xffebebeb),
    disabled: Color(0x00ffffff),
  );
}

class FluentxControlAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;
  final Color selected;

  const FluentxControlAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
    required this.selected,
  });

  factory FluentxControlAccentColors.dark(FluentxAccentColors accent) => .new(
    primary: accent.light2,
    secondary: accent.light2.withAlpha((0.9 * 255).toInt()),
    tertiary: accent.light2.withAlpha((0.8 * 255).toInt()),
    disabled: Color(0x28ffffff),
    selected: accent.base,
  );

  factory FluentxControlAccentColors.light(FluentxAccentColors accent) => .new(
    primary: accent.dark2,
    secondary: accent.dark2.withAlpha((0.9 * 255).toInt()),
    tertiary: accent.dark2.withAlpha((0.8 * 255).toInt()),
    disabled: Color(0x37000000),
    selected: accent.base,
  );
}

class FluentxStrokeControlColors {
  final Color primary;
  final Color secondary;
  final Color onAccentPrimary;
  final Color onAccentSecondary;
  final Color onAccentTertiary;
  final Color onAccentDisabled;
  final Color onImage;

  const FluentxStrokeControlColors({
    required this.primary,
    required this.secondary,
    required this.onAccentPrimary,
    required this.onAccentSecondary,
    required this.onAccentTertiary,
    required this.onAccentDisabled,
    required this.onImage,
  });

  static const dark = FluentxStrokeControlColors(
    primary: Color(0x12ffffff),
    secondary: Color(0x18ffffff),
    onAccentPrimary: Color(0x14ffffff),
    onAccentSecondary: Color(0x23000000),
    onAccentTertiary: Color(0x37000000),
    onAccentDisabled: Color(0x33000000),
    onImage: Color(0x6b000000),
  );

  static const light = FluentxStrokeControlColors(
    primary: Color(0x0f000000),
    secondary: Color(0x29000000),
    onAccentPrimary: Color(0x14ffffff),
    onAccentSecondary: Color(0x66000000),
    onAccentTertiary: Color(0x37000000),
    onAccentDisabled: Color(0x0f000000),
    onImage: Color(0x59ffffff),
  );
}

class FluentxStrokeCardColors {
  final Color primaryTransparent;
  final Color primaryOpaque;

  const FluentxStrokeCardColors({
    required this.primaryTransparent,
    required this.primaryOpaque,
  });

  static const dark = FluentxStrokeCardColors(
    primaryTransparent: Color(0x19000000),
    primaryOpaque: Color(0xff1c1c1c),
  );

  static const light = FluentxStrokeCardColors(
    primaryTransparent: Color(0x0f000000),
    primaryOpaque: Color(0xffebebeb),
  );
}

class FluentxStrokeSurfaceColors {
  final Color primary;
  final Color snapped;
  final Color flyout;

  const FluentxStrokeSurfaceColors({
    required this.primary,
    required this.snapped,
    required this.flyout,
  });

  static const dark = FluentxStrokeSurfaceColors(
    primary: Color(0x66757575),
    snapped: Color(0xff757575),
    flyout: Color(0x33000000),
  );

  static const light = FluentxStrokeSurfaceColors(
    primary: Color(0x66757575),
    snapped: Color(0xff757575),
    flyout: Color(0x0f000000),
  );
}

class FluentxStrokeDividerColors {
  final Color primary;

  const FluentxStrokeDividerColors({required this.primary});

  static const dark = FluentxStrokeDividerColors(primary: Color(0x15ffffff));
  static const light = FluentxStrokeDividerColors(primary: Color(0x0f000000));
}

class FluentxStrokeFocusColors {
  final Color outer;
  final Color inner;

  const FluentxStrokeFocusColors({required this.outer, required this.inner});

  static const dark = FluentxStrokeFocusColors(
    outer: Color(0xffffffff),
    inner: Color(0xb3000000),
  );

  static const light = FluentxStrokeFocusColors(
    outer: Color(0xe4000000),
    inner: Color(0xb3ffffff),
  );
}

class FluentxBackgroundCardColors {
  final Color primary;
  final Color secondary;
  final Color tertiary; // coded red

  const FluentxBackgroundCardColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
  });

  static const dark = FluentxBackgroundCardColors(
    primary: Color(0x0dffffff),
    secondary: Color(0x08ffffff),
    tertiary: Color(0x12ffffff),
  );

  static const light = FluentxBackgroundCardColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x80f6f6f6),
    tertiary: Color(0xffffffff),
  );
}

class FluentxBackgroundSmokeColors {
  final Color primary;

  const FluentxBackgroundSmokeColors({required this.primary});

  static const dark = FluentxBackgroundSmokeColors(primary: Color(0x4d000000));

  static const light = FluentxBackgroundSmokeColors(primary: Color(0x4d000000));
}

class FluentxBackgroundLayerColors {
  final Color primary;
  final Color secondary;

  const FluentxBackgroundLayerColors({
    required this.primary,
    required this.secondary,
  });

  static const dark = FluentxBackgroundLayerColors(
    primary: Color(0x4c3a3a3a),
    secondary: Color(0x0dffffff),
  );

  static const light = FluentxBackgroundLayerColors(
    primary: Color(0x80ffffff),
    secondary: Color(0xffffffff),
  );
}

class FluentxBackgroundLayerOnAcrylicColors {
  final Color primary;

  const FluentxBackgroundLayerOnAcrylicColors({required this.primary});

  static const dark = FluentxBackgroundLayerOnAcrylicColors(
    primary: Color(0x09ffffff),
  );

  static const light = FluentxBackgroundLayerOnAcrylicColors(
    primary: Color(0x40ffffff),
  );
}

class FluentxBackgroundLayerOnMicaAltColors {
  final Color primary; // coded red
  final Color secondary; // coded red
  final Color tertiary; // coded red
  final Color transparent; // coded red

  const FluentxBackgroundLayerOnMicaAltColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.transparent,
  });

  static const dark = FluentxBackgroundLayerOnMicaAltColors(
    primary: Color(0x733a3a3a),
    secondary: Color(0x0fffffff),
    tertiary: Color(0xfff9f9f9),
    transparent: Color(0x00000000),
  );

  static const light = FluentxBackgroundLayerOnMicaAltColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x0a000000),
    tertiary: Color(0xfff9f9f9),
    transparent: Color(0x00ffffff),
  );
}

class FluentxBackgroundSolidColors {
  final Color primary;
  final Color alternative; // coded red
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color quinary; // coded red
  final Color senary; // coded red

  const FluentxBackgroundSolidColors({
    required this.primary,
    required this.alternative,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.quinary,
    required this.senary,
  });

  static const dark = FluentxBackgroundSolidColors(
    primary: Color(0xff202020),
    alternative: Color(0xff0a0a0a),
    secondary: Color(0xff1c1c1c),
    tertiary: Color(0xff282828),
    quarternary: Color(0xff2c2c2c),
    quinary: Color(0xff333333),
    senary: Color(0xff373737),
  );

  static const light = FluentxBackgroundSolidColors(
    primary: Color(0xfff3f3f3),
    alternative: Color(0xffdadada),
    secondary: Color(0xffeeeeee),
    tertiary: Color(0xfff9f9f9),
    quarternary: Color(0xffffffff),
    quinary: Color(0xfffdfdfd),
    senary: Color(0xffffffff),
  );
}
