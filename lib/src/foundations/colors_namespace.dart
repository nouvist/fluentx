part of 'colors.dart';

class FluentxForegroundColors {
  final FluentxForegroundPrimaryColors primary;
  final FluentxForegroundAccentColors accent;
  final FluentxForegroundOnAccentColors onAccent;

  const FluentxForegroundColors({
    required this.primary,
    required this.accent,
    required this.onAccent,
  });

  factory FluentxForegroundColors.dark(FluentxAccentColors accent) =>
      .new(primary: .dark, accent: .dark(accent), onAccent: .dark);

  factory FluentxForegroundColors.light(FluentxAccentColors accent) =>
      .new(primary: .light, accent: .light(accent), onAccent: .light);
}

class FluentxControlColors {
  final FluentxControlPrimaryColors primary;
  final FluentxControlSecondaryColors secondary;
  final FluentxControlSolidColors solid;
  final FluentxControlStrongColors strong;
  final FluentxControlSubtleColors subtle;
  final FluentxControlOnImageColors onImage;

  const FluentxControlColors({
    required this.primary,
    required this.secondary,
    required this.solid,
    required this.strong,
    required this.subtle,
    required this.onImage,
  });

  static const dark = FluentxControlColors(
    primary: .dark,
    secondary: .dark,
    solid: .dark,
    strong: .dark,
    subtle: .dark,
    onImage: .dark,
  );

  static const light = FluentxControlColors(
    primary: .light,
    secondary: .light,
    solid: .light,
    strong: .light,
    subtle: .light,
    onImage: .light,
  );
}

class FluentxStrokeColors {
  final FluentxStrokeControlColors control;
  final FluentxStrokeCardColors card;
  final FluentxStrokeSurfaceColors surface;
  final FluentxStrokeDividerColors divider;
  final FluentxStrokeFocusColors focus;

  const FluentxStrokeColors({
    required this.control,
    required this.card,
    required this.surface,
    required this.divider,
    required this.focus,
  });

  static const dark = FluentxStrokeColors(
    control: .dark,
    card: .dark,
    surface: .dark,
    divider: .dark,
    focus: .dark,
  );

  static const light = FluentxStrokeColors(
    control: .light,
    card: .light,
    surface: .light,
    divider: .light,
    focus: .light,
  );
}

class FluentxBackgroundColors {
  final FluentxBackgroundCardColors card;
  final FluentxBackgroundSmokeColors smoke;
  final FluentxBackgroundLayerColors layer;
  final FluentxBackgroundSolidColors solid;

  const FluentxBackgroundColors({
    required this.card,
    required this.smoke,
    required this.layer,
    required this.solid,
  });

  static const dark = FluentxBackgroundColors(
    card: .dark,
    smoke: .dark,
    layer: .dark,
    solid: .dark,
  );

  static const light = FluentxBackgroundColors(
    card: .light,
    smoke: .light,
    layer: .light,
    solid: .light,
  );
}

class FluentxSystemColors {
  final FluentxSystemItemColors attention;
  final FluentxSystemItemColors success;
  final FluentxSystemItemColors caution;
  final FluentxSystemItemColors critical;
  final FluentxSystemItemColors neutral;
  final FluentxSystemItemColors neutralSolid;

  const FluentxSystemColors({
    required this.attention,
    required this.success,
    required this.caution,
    required this.critical,
    required this.neutral,
    required this.neutralSolid,
  });

  static const dark = FluentxSystemColors(
    attention: .attentionDark,
    success: .successDark,
    caution: .cautionDark,
    critical: .criticalDark,
    neutral: .neutralDark,
    neutralSolid: .neutralSolidDark,
  );

  static const light = FluentxSystemColors(
    attention: .attentionLight,
    success: .successLight,
    caution: .cautionLight,
    critical: .criticalLight,
    neutral: .neutralLight,
    neutralSolid: .neutralSolidLight,
  );
}
