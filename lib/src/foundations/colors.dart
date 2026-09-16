import 'package:fluentx/src/rust/foundations/colors.dart';
import 'package:flutter/services.dart';

part 'colors_namespace.dart';
part 'colors_implementation.dart';

class FxColors {
  final Brightness brightness;
  final FxAccentColors accent;
  final FxForegroundColors foreground;
  final FxControlColors control;
  final FxStrokeColors stroke;
  final FxBackgroundColors background;
  final FxSystemColors system;

  const FxColors({
    required this.brightness,
    required this.accent,
    required this.foreground,
    required this.control,
    required this.stroke,
    required this.background,
    required this.system,
  });

  factory FxColors.current() {
    final native = FxNativeBrightness.current();
    if (native == null) return .light(.current());
    return switch (native) {
      .dark => .dark(.current()),
      .light => .light(.current()),
    };
  }

  factory FxColors.dark(FxAccentColors accent) => .new(
    brightness: .dark,
    accent: accent,
    foreground: .dark(accent),
    control: .dark,
    stroke: .dark,
    background: .dark,
    system: .dark,
  );

  factory FxColors.light(FxAccentColors accent) => .new(
    brightness: .light,
    accent: accent,
    foreground: .light(accent),
    control: .light,
    stroke: .light,
    background: .light,
    system: .light,
  );
}
