part of 'colors.dart';

class FxForegroundColors {
  final FxForegroundPrimaryColors primary;
  final FxForegroundAccentColors accent;
  final FxForegroundOnAccentColors onAccent;

  const FxForegroundColors({
    required this.primary,
    required this.accent,
    required this.onAccent,
  });

  factory FxForegroundColors.dark(FxAccentColors accent) =>
      .new(primary: .dark, accent: .dark(accent), onAccent: .dark);

  factory FxForegroundColors.light(FxAccentColors accent) =>
      .new(primary: .light, accent: .light(accent), onAccent: .light);
}

class FxControlColors {
  final FxControlPrimaryColors primary;
  final FxControlSecondaryColors secondary;
  final FxControlSolidColors solid;
  final FxControlStrongColors strong;
  final FxControlSubtleColors subtle;
  final FxControlOnImageColors onImage;

  const FxControlColors({
    required this.primary,
    required this.secondary,
    required this.solid,
    required this.strong,
    required this.subtle,
    required this.onImage,
  });

  static const dark = FxControlColors(
    primary: .dark,
    secondary: .dark,
    solid: .dark,
    strong: .dark,
    subtle: .dark,
    onImage: .dark,
  );

  static const light = FxControlColors(
    primary: .light,
    secondary: .light,
    solid: .light,
    strong: .light,
    subtle: .light,
    onImage: .light,
  );
}

class FxStrokeColors {
  final FxStrokeControlColors control;
  final FxStrokeCardColors card;
  final FxStrokeSurfaceColors surface;
  final FxStrokeDividerColors divider;
  final FxStrokeFocusColors focus;

  const FxStrokeColors({
    required this.control,
    required this.card,
    required this.surface,
    required this.divider,
    required this.focus,
  });

  static const dark = FxStrokeColors(
    control: .dark,
    card: .dark,
    surface: .dark,
    divider: .dark,
    focus: .dark,
  );

  static const light = FxStrokeColors(
    control: .light,
    card: .light,
    surface: .light,
    divider: .light,
    focus: .light,
  );
}

class FxBackgroundColors {
  final FxBackgroundCardColors card;
  final FxBackgroundSmokeColors smoke;
  final FxBackgroundLayerColors layer;
  final FxBackgroundSolidColors solid;

  const FxBackgroundColors({
    required this.card,
    required this.smoke,
    required this.layer,
    required this.solid,
  });

  static const dark = FxBackgroundColors(
    card: .dark,
    smoke: .dark,
    layer: .dark,
    solid: .dark,
  );

  static const light = FxBackgroundColors(
    card: .light,
    smoke: .light,
    layer: .light,
    solid: .light,
  );
}

class FxSystemColors {
  final FxSystemItemColors attention;
  final FxSystemItemColors success;
  final FxSystemItemColors caution;
  final FxSystemItemColors critical;
  final FxSystemItemColors neutral;
  final FxSystemItemColors neutralSolid;

  const FxSystemColors({
    required this.attention,
    required this.success,
    required this.caution,
    required this.critical,
    required this.neutral,
    required this.neutralSolid,
  });

  static const dark = FxSystemColors(
    attention: .attentionDark,
    success: .successDark,
    caution: .cautionDark,
    critical: .criticalDark,
    neutral: .neutralDark,
    neutralSolid: .neutralSolidDark,
  );

  static const light = FxSystemColors(
    attention: .attentionLight,
    success: .successLight,
    caution: .cautionLight,
    critical: .criticalLight,
    neutral: .neutralLight,
    neutralSolid: .neutralSolidLight,
  );
}
