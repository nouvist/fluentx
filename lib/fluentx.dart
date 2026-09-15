library;

import 'package:fluentx/src/rust/frb_generated.dart';

export 'package:flutter/widgets.dart';

export 'src/foundations.dart';
export 'src/widgets.dart';

abstract final class Fluentx {
  static Future<void> init() => RustLib.init();
}
