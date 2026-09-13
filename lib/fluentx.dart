library;

import 'package:fluentx/src/rust/frb_generated.dart';

export 'src/rust/api/simple.dart';

abstract final class Fluentx {
  static Future<void> init() => RustLib.init();
}
