part of 'colors.dart';

class FxAccentColors {
  final Color light3;
  final Color light2;
  final Color light1;
  final Color base;
  final Color dark1;
  final Color dark2;
  final Color dark3;

  const FxAccentColors({
    required this.light3,
    required this.light2,
    required this.light1,
    required this.base,
    required this.dark1,
    required this.dark2,
    required this.dark3,
  });

  factory FxAccentColors.current() {
    final native = FxNativeColors.current();
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

  static const fallback = FxAccentColors(
    light3: Color(0xff99ebff),
    light2: Color(0xff4cc2ff),
    light1: Color(0xff0091f8),
    base: Color(0xff0078d4),
    dark1: Color(0xff0067c0),
    dark2: Color(0xff003e92),
    dark3: Color(0xff001a68),
  );
}

class FxForegroundPrimaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FxForegroundPrimaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FxForegroundPrimaryColors(
    primary: Color(0xffffffff),
    secondary: Color(0xc5ffffff),
    tertiary: Color(0x87ffffff),
    disabled: Color(0x5dffffff),
  );

  static const light = FxForegroundPrimaryColors(
    primary: Color(0xe4000000),
    secondary: Color(0x9e000000),
    tertiary: Color(0x72000000),
    disabled: Color(0x5c000000),
  );
}

class FxForegroundAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FxForegroundAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  factory FxForegroundAccentColors.dark(FxAccentColors accent) =>
      .new(
        primary: accent.light3,
        secondary: accent.light3,
        tertiary: accent.light2,
        disabled: const Color(0x5dffffff),
      );

  factory FxForegroundAccentColors.light(FxAccentColors accent) =>
      .new(
        primary: accent.dark2,
        secondary: accent.dark3,
        tertiary: accent.dark1,
        disabled: const Color(0x5c000000),
      );
}

class FxForegroundOnAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FxForegroundOnAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FxForegroundOnAccentColors(
    primary: Color(0xff000000),
    secondary: Color(0x80000000),
    tertiary: Color(0x87000000),
    disabled: Color(0xffffffff),
  );

  static const light = FxForegroundOnAccentColors(
    primary: Color(0xffffffff),
    secondary: Color(0xb3ffffff),
    tertiary: Color(0xb3ffffff),
    disabled: Color(0xffffffff),
  );
}

class FxControlPrimaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color disabled;
  final Color transparent;
  final Color active;

  const FxControlPrimaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.disabled,
    required this.transparent,
    required this.active,
  });

  static const dark = FxControlPrimaryColors(
    primary: Color(0x0fffffff),
    secondary: Color(0x15ffffff),
    tertiary: Color(0x0bffffff),
    quarternary: Color(0x0fffffff),
    disabled: Color(0x4df9f9f9),
    transparent: Color(0x00ffffff),
    active: Color(0xb31e1e1e),
  );

  static const light = FxControlPrimaryColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x80f9f9f9),
    tertiary: Color(0x4df9f9f9),
    quarternary: Color(0xc2f3f3f3),
    disabled: Color(0x4df9f9f9),
    transparent: Color(0x00ffffff),
    active: Color(0xffffffff),
  );
}

class FxControlSecondaryColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color disabled;

  const FxControlSecondaryColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.disabled,
  });

  static const dark = FxControlSecondaryColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x19000000),
    tertiary: Color(0x0bffffff),
    quarternary: Color(0x12ffffff),
    disabled: Color(0x00ffffff),
  );

  static const light = FxControlSecondaryColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x06000000),
    tertiary: Color(0x0f000000),
    quarternary: Color(0x18000000),
    disabled: Color(0x00ffffff),
  );
}

class FxControlSolidColors {
  final Color primary;

  const FxControlSolidColors({required this.primary});

  static const dark = FxControlSolidColors(primary: Color(0xff454545));
  static const light = FxControlSolidColors(primary: Color(0xffffffff));
}

class FxControlStrongColors {
  final Color primary;
  final Color disabled;

  const FxControlStrongColors({
    required this.primary,
    required this.disabled,
  });

  static const dark = FxControlStrongColors(
    primary: Color(0x8bffffff),
    disabled: Color(0x3fffffff),
  );

  static const light = FxControlStrongColors(
    primary: Color(0x72000000),
    disabled: Color(0x51000000),
  );
}

class FxControlSubtleColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FxControlSubtleColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FxControlSubtleColors(
    primary: Color(0x00ffffff),
    secondary: Color(0x0fffffff),
    tertiary: Color(0x0affffff),
    disabled: Color(0x00ffffff),
  );

  static const light = FxControlSubtleColors(
    primary: Color(0x00000000),
    secondary: Color(0x09000000),
    tertiary: Color(0x06000000),
    disabled: Color(0x00000000),
  );
}

class FxControlOnImageColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;

  const FxControlOnImageColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
  });

  static const dark = FxControlOnImageColors(
    primary: Color(0xb31c1c1c),
    secondary: Color(0xff1a1a1a),
    tertiary: Color(0xff131313),
    disabled: Color(0xff1e1e1e),
  );

  static const light = FxControlOnImageColors(
    primary: Color(0xc9ffffff),
    secondary: Color(0xfff3f3f3),
    tertiary: Color(0xffebebeb),
    disabled: Color(0x00ffffff),
  );
}

class FxControlAccentColors {
  final Color primary;
  final Color secondary;
  final Color tertiary;
  final Color disabled;
  final Color selected;

  const FxControlAccentColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.disabled,
    required this.selected,
  });

  factory FxControlAccentColors.dark(FxAccentColors accent) => .new(
    primary: accent.light2,
    secondary: accent.light2.withAlpha((0.9 * 255).toInt()),
    tertiary: accent.light2.withAlpha((0.8 * 255).toInt()),
    disabled: Color(0x28ffffff),
    selected: accent.base,
  );

  factory FxControlAccentColors.light(FxAccentColors accent) => .new(
    primary: accent.dark2,
    secondary: accent.dark2.withAlpha((0.9 * 255).toInt()),
    tertiary: accent.dark2.withAlpha((0.8 * 255).toInt()),
    disabled: Color(0x37000000),
    selected: accent.base,
  );
}

class FxStrokeControlColors {
  final Color primary;
  final Color secondary;
  final Color onAccentPrimary;
  final Color onAccentSecondary;
  final Color onAccentTertiary;
  final Color onAccentDisabled;
  final Color onImage;

  const FxStrokeControlColors({
    required this.primary,
    required this.secondary,
    required this.onAccentPrimary,
    required this.onAccentSecondary,
    required this.onAccentTertiary,
    required this.onAccentDisabled,
    required this.onImage,
  });

  static const dark = FxStrokeControlColors(
    primary: Color(0x12ffffff),
    secondary: Color(0x18ffffff),
    onAccentPrimary: Color(0x14ffffff),
    onAccentSecondary: Color(0x23000000),
    onAccentTertiary: Color(0x37000000),
    onAccentDisabled: Color(0x33000000),
    onImage: Color(0x6b000000),
  );

  static const light = FxStrokeControlColors(
    primary: Color(0x0f000000),
    secondary: Color(0x29000000),
    onAccentPrimary: Color(0x14ffffff),
    onAccentSecondary: Color(0x66000000),
    onAccentTertiary: Color(0x37000000),
    onAccentDisabled: Color(0x0f000000),
    onImage: Color(0x59ffffff),
  );
}

class FxStrokeCardColors {
  final Color primaryTransparent;
  final Color primaryOpaque;

  const FxStrokeCardColors({
    required this.primaryTransparent,
    required this.primaryOpaque,
  });

  static const dark = FxStrokeCardColors(
    primaryTransparent: Color(0x19000000),
    primaryOpaque: Color(0xff1c1c1c),
  );

  static const light = FxStrokeCardColors(
    primaryTransparent: Color(0x0f000000),
    primaryOpaque: Color(0xffebebeb),
  );
}

class FxStrokeSurfaceColors {
  final Color primary;
  final Color snapped;
  final Color flyout;

  const FxStrokeSurfaceColors({
    required this.primary,
    required this.snapped,
    required this.flyout,
  });

  static const dark = FxStrokeSurfaceColors(
    primary: Color(0x66757575),
    snapped: Color(0xff757575),
    flyout: Color(0x33000000),
  );

  static const light = FxStrokeSurfaceColors(
    primary: Color(0x66757575),
    snapped: Color(0xff757575),
    flyout: Color(0x0f000000),
  );
}

class FxStrokeDividerColors {
  final Color primary;

  const FxStrokeDividerColors({required this.primary});

  static const dark = FxStrokeDividerColors(primary: Color(0x15ffffff));
  static const light = FxStrokeDividerColors(primary: Color(0x0f000000));
}

class FxStrokeFocusColors {
  final Color outer;
  final Color inner;

  const FxStrokeFocusColors({required this.outer, required this.inner});

  static const dark = FxStrokeFocusColors(
    outer: Color(0xffffffff),
    inner: Color(0xb3000000),
  );

  static const light = FxStrokeFocusColors(
    outer: Color(0xe4000000),
    inner: Color(0xb3ffffff),
  );
}

class FxBackgroundCardColors {
  final Color primary;
  final Color secondary;
  final Color tertiary; // coded red

  const FxBackgroundCardColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
  });

  static const dark = FxBackgroundCardColors(
    primary: Color(0x0dffffff),
    secondary: Color(0x08ffffff),
    tertiary: Color(0x12ffffff),
  );

  static const light = FxBackgroundCardColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x80f6f6f6),
    tertiary: Color(0xffffffff),
  );
}

class FxBackgroundSmokeColors {
  final Color primary;

  const FxBackgroundSmokeColors({required this.primary});

  static const dark = FxBackgroundSmokeColors(primary: Color(0x4d000000));

  static const light = FxBackgroundSmokeColors(primary: Color(0x4d000000));
}

class FxBackgroundLayerColors {
  final Color primary;
  final Color secondary;

  const FxBackgroundLayerColors({
    required this.primary,
    required this.secondary,
  });

  static const dark = FxBackgroundLayerColors(
    primary: Color(0x4c3a3a3a),
    secondary: Color(0x0dffffff),
  );

  static const light = FxBackgroundLayerColors(
    primary: Color(0x80ffffff),
    secondary: Color(0xffffffff),
  );
}

class FxBackgroundLayerOnAcrylicColors {
  final Color primary;

  const FxBackgroundLayerOnAcrylicColors({required this.primary});

  static const dark = FxBackgroundLayerOnAcrylicColors(
    primary: Color(0x09ffffff),
  );

  static const light = FxBackgroundLayerOnAcrylicColors(
    primary: Color(0x40ffffff),
  );
}

class FxBackgroundLayerOnMicaAltColors {
  final Color primary; // coded red
  final Color secondary; // coded red
  final Color tertiary; // coded red
  final Color transparent; // coded red

  const FxBackgroundLayerOnMicaAltColors({
    required this.primary,
    required this.secondary,
    required this.tertiary,
    required this.transparent,
  });

  static const dark = FxBackgroundLayerOnMicaAltColors(
    primary: Color(0x733a3a3a),
    secondary: Color(0x0fffffff),
    tertiary: Color(0xfff9f9f9),
    transparent: Color(0x00000000),
  );

  static const light = FxBackgroundLayerOnMicaAltColors(
    primary: Color(0xb3ffffff),
    secondary: Color(0x0a000000),
    tertiary: Color(0xfff9f9f9),
    transparent: Color(0x00ffffff),
  );
}

class FxBackgroundSolidColors {
  final Color primary;
  final Color alternative; // coded red
  final Color secondary;
  final Color tertiary;
  final Color quarternary;
  final Color quinary; // coded red
  final Color senary; // coded red

  const FxBackgroundSolidColors({
    required this.primary,
    required this.alternative,
    required this.secondary,
    required this.tertiary,
    required this.quarternary,
    required this.quinary,
    required this.senary,
  });

  static const dark = FxBackgroundSolidColors(
    primary: Color(0xff202020),
    alternative: Color(0xff0a0a0a),
    secondary: Color(0xff1c1c1c),
    tertiary: Color(0xff282828),
    quarternary: Color(0xff2c2c2c),
    quinary: Color(0xff333333),
    senary: Color(0xff373737),
  );

  static const light = FxBackgroundSolidColors(
    primary: Color(0xfff3f3f3),
    alternative: Color(0xffdadada),
    secondary: Color(0xffeeeeee),
    tertiary: Color(0xfff9f9f9),
    quarternary: Color(0xffffffff),
    quinary: Color(0xfffdfdfd),
    senary: Color(0xffffffff),
  );
}

class FxSystemItemColors {
  final Color foreground;
  final Color background;

  const FxSystemItemColors({
    required this.foreground,
    required this.background,
  });

  static const attentionDark = FxSystemItemColors(
    foreground: Color(0xff60cdff),
    background: Color(0xff2e2e2e),
  );
  static const successDark = FxSystemItemColors(
    foreground: Color(0xff6ccb5f),
    background: Color(0xff393d1b),
  );
  static const cautionDark = FxSystemItemColors(
    foreground: Color(0xfffce100),
    background: Color(0xff433519),
  );
  static const criticalDark = FxSystemItemColors(
    foreground: Color(0xffff99a4),
    background: Color(0xff442726),
  );
  static const neutralDark = FxSystemItemColors(
    foreground: Color(0x8bffffff),
    background: Color(0x08ffffff),
  );
  static const neutralSolidDark = FxSystemItemColors(
    foreground: Color(0xff9d9d9d),
    background: Color(0xff2e2e2e),
  );

  static const attentionLight = FxSystemItemColors(
    foreground: Color(0xff0070cb),
    background: Color(0xfff7f7f7),
  );
  static const successLight = FxSystemItemColors(
    foreground: Color(0xff0f7b0f),
    background: Color(0xffdff6dd),
  );
  static const cautionLight = FxSystemItemColors(
    foreground: Color(0xff9d5d00),
    background: Color(0xfffff4ce),
  );
  static const criticalLight = FxSystemItemColors(
    foreground: Color(0xffc42b1c),
    background: Color(0xfffde7e9),
  );
  static const neutralLight = FxSystemItemColors(
    foreground: Color(0x72000000),
    background: Color(0x06000000),
  );
  static const neutralSolidLight = FxSystemItemColors(
    foreground: Color(0xff8a8a8a),
    background: Color(0xfff3f3f3),
  );
}
