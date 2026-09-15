import 'package:fluentx/src/rust/foundations/colors.dart';
import 'package:flutter/services.dart';

part 'colors_namespace.dart';
part 'colors_implementation.dart';

class FluentxColors {
  final Brightness brightness;
  final FluentxAccentColors accent;
  final FluentxForegroundColors foreground;
  final FluentxControlColors control;
  final FluentxStrokeColors stroke;
  final FluentxBackgroundColors background;
  final FluentxSystemColors system;

  const FluentxColors({
    required this.brightness,
    required this.accent,
    required this.foreground,
    required this.control,
    required this.stroke,
    required this.background,
    required this.system,
  });

  factory FluentxColors.current() {
    final native = FluentxNativeBrightness.current();
    if (native == null) return .light(.current());
    return switch (native) {
      .dark => .dark(.current()),
      .light => .light(.current()),
    };
  }

  factory FluentxColors.dark(FluentxAccentColors accent) => .new(
    brightness: .dark,
    accent: accent,
    foreground: .dark(accent),
    control: .dark,
    stroke: .dark,
    background: .dark,
    system: .dark,
  );

  factory FluentxColors.light(FluentxAccentColors accent) => .new(
    brightness: .light,
    accent: accent,
    foreground: .light(accent),
    control: .light,
    stroke: .light,
    background: .light,
    system: .light,
  );
}
