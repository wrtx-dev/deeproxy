#[cfg(target_os = "macos")]
use objc2::rc::autoreleasepool;
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

#[allow(dead_code)]
pub enum ActivationPolicy {
    Regular,
    Accessory,
    Prohibited,
}

pub fn set_activation_policy(policy: ActivationPolicy) {
    autoreleasepool(|_| {
        let mtm = MainThreadMarker::new().unwrap();
        #[allow(unsafe_code, unused_unsafe)]
        let app = unsafe { NSApplication::sharedApplication(mtm) };

        unsafe {
            match policy {
                ActivationPolicy::Regular => {
                    app.setActivationPolicy(objc2_app_kit::NSApplicationActivationPolicy(0));
                    app.activate();
                }
                ActivationPolicy::Accessory => {
                    app.setActivationPolicy(objc2_app_kit::NSApplicationActivationPolicy(1));
                    app.activate();
                }
                ActivationPolicy::Prohibited => {
                    app.setActivationPolicy(objc2_app_kit::NSApplicationActivationPolicy(2));
                    app.hide(None);
                }
            }
        }
    });
}
