import 'dart:async';

export 'foundations/colors.dart';
export 'foundations/backdrop.dart';

Future<void> yieldNow([FutureOr<void> Function()? callback]) =>
    .delayed(.zero, callback);
