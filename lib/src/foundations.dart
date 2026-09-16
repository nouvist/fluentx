import 'dart:async';

import 'package:flutter/foundation.dart';

export 'foundations/colors.dart';
export 'foundations/backdrop.dart';

Future<void> yieldNow([FutureOr<void> Function()? callback]) {
  return .delayed(.zero, callback);
}

var _isWindows = null as bool?;
bool get isWindows {
  return _isWindows ??= !kIsWeb && defaultTargetPlatform == .windows;
}
