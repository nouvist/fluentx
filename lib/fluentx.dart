library;

import 'package:fluentx/src/rust/frb_generated.dart';

export 'src/foundations/colors.dart';

abstract final class Fluentx {
  static Future<void> init() => RustLib.init();
}
