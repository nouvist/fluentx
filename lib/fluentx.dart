library;

import 'package:fluentx/src/rust/frb_generated.dart';

export 'src/foundations/colors.dart';
export 'src/foundations/backdrop.dart';

abstract final class Fluentx {
  static Future<void> init() => RustLib.init();
}
