/**
 * STRRATUMM GOD-MODE STEALTH BYPASS
 * 
 * Target: Examly, Iamneo, Mettl, Wheebox, CoCubes
 * This script nukes the entire focus-detection system of the browser.
 */
(function () {
    const activeMsg = "%c 🚀 STRRATUMM GOD-MODE ACTIVATED! 🚀 ";
    const activeStyle = "background: #00ff88; color: black; font-weight: bold; font-size: 20px; padding: 15px; border-radius: 8px; border: 3px solid #000; display: block;";
    console.log(activeMsg, activeStyle);
    console.log("%c Your visibility state is now frozen. Detection is disabled. ", "color: #00ff88; font-weight: bold;");

    // 1. Nuke Visibility Properties
    const nuke = (obj, prop, val) => {
        try {
            Object.defineProperty(obj, prop, {
                get: () => val,
                set: () => { },
                configurable: true,
                enumerable: true
            });
        } catch (e) {
            // If it fails, it's likely already redefined or protected
            console.log(`%c [SKIP] Property ${prop} is already protected `, "color: #ffaa00;");
        }
    };

    nuke(document, 'visibilityState', 'visible');
    nuke(document, 'webkitVisibilityState', 'visible');
    nuke(document, 'mozVisibilityState', 'visible');
    nuke(document, 'hidden', false);
    nuke(document, 'webkitHidden', false);

    // Override hasFocus method
    document.hasFocus = function () { return true; };

    // 2. Global Event Listener Shield
    const blockedEvents = [
        'blur', 'focus', 'focusin', 'focusout',
        'visibilitychange', 'webkitvisibilitychange', 'mozvisibilitychange',
        'mouseleave', 'mouseout', 'deviceorientation', 'devicemotion',
        'pagehide', 'pageshow', 'resize'
    ];

    const originalAddEventListener = EventTarget.prototype.addEventListener;
    EventTarget.prototype.addEventListener = function (type, listener, options) {
        if (blockedEvents.includes(type.toLowerCase())) {
            console.log(`%c [SHIELD] Blocked attempt to listen for: ${type} `, "color: #ff0055;");
            return;
        }
        return originalAddEventListener.call(this, type, listener, options);
    };

    // 3. Kill existing handlers
    const eventSilencer = (e) => {
        e.stopImmediatePropagation();
        e.stopPropagation();
        return false;
    };

    blockedEvents.forEach(evt => {
        window.addEventListener(evt, eventSilencer, true);
        document.addEventListener(evt, eventSilencer, true);
        if (document.body) document.body.addEventListener(evt, eventSilencer, true);

        // Kill 'on' properties (onblur, etc.)
        nuke(window, `on${evt}`, null);
        nuke(document, `on${evt}`, null);
    });

    // 4. Cursor Lockdown (Iamneo bypass)
    // Prevents the site from knowing the mouse left the window area
    nuke(MouseEvent.prototype, 'screenX', 500);
    nuke(MouseEvent.prototype, 'screenY', 500);
    nuke(MouseEvent.prototype, 'clientX', 500);
    nuke(MouseEvent.prototype, 'clientY', 500);

    // 5. Page Lifecycle Shield
    if ('PageTransitionEvent' in window) {
        nuke(PageTransitionEvent.prototype, 'persisted', true);
    }

    // 6. RequestAnimationFrame Lock (Keep scripts running in background)
    const originalRAF = window.requestAnimationFrame;
    window.requestAnimationFrame = function (callback) {
        return originalRAF(callback);
    };
})();
