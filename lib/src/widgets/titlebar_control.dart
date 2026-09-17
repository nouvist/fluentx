import 'dart:async';

import 'package:fluentx/fluentx.dart';
import 'package:fluentx/src/rust/foundations/window.dart';
import 'package:flutter_svg/flutter_svg.dart';

class FxTitlebarControl extends StatelessWidget {
  const FxTitlebarControl({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 144,
      height: 48,
      child: Row(
        children: [
          Expanded(child: _Item(.minimize)),
          Expanded(child: _Item(.maximize)),
          Expanded(child: _Item(.close)),
        ],
      ),
    );
  }
}

enum _ItemVariant {
  close,
  maximize,
  minimize;

  static _ItemVariant? fromHit(FxNativeWindowHitEvent event) => switch (event) {
    .close => .close,
    .maximize => .maximize,
    .minimize => .minimize,
    _ => null,
  };
}

class _Item extends StatefulWidget {
  final _ItemVariant variant;

  const _Item(this.variant);

  @override
  State<_Item> createState() => _ItemState();
}

class _ItemState extends State<_Item> {
  final _key = GlobalKey();

  late int _hit;
  late int _mouse;
  late int _change;
  var _isHover = false;
  var _isPressed = false;

  @override
  void initState() {
    super.initState();
    _hit = FxNativeWindow.instance().addHitListener(callback: _handleHit);
    _hit = FxNativeWindow.instance().addMouseListener(callback: _handleMouse);
    _change = FxNativeWindow.instance().addListener(callback: _handleChange);
  }

  @override
  void dispose() {
    FxNativeWindow.instance().removeHitListener(id: _hit);
    FxNativeWindow.instance().removeMouseListener(id: _mouse);
    FxNativeWindow.instance().removeListener(id: _change);
    super.dispose();
  }

  Future<void> _handleChange(FxNativeWindowEvent event) async {
    if (event == .maximize && widget.variant == .maximize) {
      await yieldNow();
      (context as StatefulElement).markNeedsBuild();
    }
  }

  Future<void> _handleHit(FxNativeWindowHitEvent event) async {
    final variant = _ItemVariant.fromHit(event);
    final next = variant == widget.variant;
    if (next == _isHover) return;

    await yieldNow();
    if (!mounted) return;
    setState(() {
      _isHover = next;
      if (next == false) _isPressed = false;
    });
  }

  Future<void> _handleMouse(FxNativeWindowMouseEvent event) async {
    if (!_isHover) return;
    await yieldNow();

    if (!mounted) return;
    setState(() {
      _isPressed = event == .down;
    });
  }

  Future<void> _handleLayout() async {
    await yieldNow();
    final box = _key.currentContext?.findRenderObject() as RenderBox?;
    if (box == null) return;

    final position = box.localToGlobal(.zero);
    final rect = FxNativeWindowRect(
      left: position.dx.toInt(),
      top: position.dy.toInt(),
      right: (position.dx + box.size.width).toInt(),
      bottom: (position.dy + box.size.height).toInt(),
    );

    switch (widget.variant) {
      case .close:
        FxNativeWindow.instance().setCloseRect(rect: rect);
        break;
      case .maximize:
        FxNativeWindow.instance().setMaximizeRect(rect: rect);
        break;
      case .minimize:
        FxNativeWindow.instance().setMinimizeRect(rect: rect);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    final t = FxTheme.of(context);
    return _LayoutSync(
      onLayout: _handleLayout,
      child: ColoredBox(
        color: switch ((_isHover, _isPressed)) {
          (true, true) => switch (widget.variant) {
            .close => switch (t.colors.brightness) {
              .dark => Color(0xe6c42b1c),
              .light => Color(0xe6c42b1c),
            },
            _ => t.colors.control.subtle.tertiary,
          },
          (true, false) => switch (widget.variant) {
            .close => switch (t.colors.brightness) {
              .dark => Color(0xffc42b1c),
              .light => Color(0xffc42b1c),
            },
            _ => t.colors.control.subtle.secondary,
          },
          _ => t.colors.control.subtle.primary,
        },
        child: Center(
          key: _key,
          child: SvgPicture.asset(
            width: 16,
            height: 16,
            colorFilter: .mode(switch (_isHover &&
                t.colors.brightness == .light &&
                widget.variant == .close) {
              true => t.colors.foreground.onAccent.primary,
              false => t.colors.foreground.primary.primary,
            }, .srcIn),
            package: 'fluentx',
            switch (widget.variant) {
              .close => 'assets/images/titlebar_controls/close.svg',
              .maximize => switch (FxNativeWindow.instance().isMaximized()) {
                true => 'assets/images/titlebar_controls/restore.svg',
                false => 'assets/images/titlebar_controls/maximize.svg',
              },
              .minimize => 'assets/images/titlebar_controls/minimize.svg',
            },
          ),
        ),
      ),
    );
  }
}

class _LayoutSync extends StatelessWidget {
  final Widget child;
  final VoidCallback onLayout;

  const _LayoutSync({required this.onLayout, required this.child});

  @override
  Widget build(BuildContext context) {
    context.dependOnInheritedWidgetOfExactType<MediaQuery>();

    onLayout();
    return child;
  }
}
