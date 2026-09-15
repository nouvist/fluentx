library;

import 'package:fluentx/src/rust/foundations/window.dart';
import 'package:fluentx/src/rust/frb_generated.dart';
import 'package:flutter/widgets.dart';

export 'package:flutter/widgets.dart';

export 'src/foundations.dart';
export 'src/widgets.dart';

abstract final class Fluentx {
  static Future<void> init() async {
    WidgetsFlutterBinding.ensureInitialized();
    await RustLib.init();
    FluentxNativeWindow.instance().refresh();
  }
}
