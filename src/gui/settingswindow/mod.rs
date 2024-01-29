// use cxx_qt::qobject;
// use cxx_qt::{bridge, qobject};
#[cxx_qt::bridge]
pub mod qt_settings_object {

    use crate::signals;

    #[cxx_qt::qobject(qml_uri = "brightcast_signals", qml_version = "1.0")]
    #[derive(Default)]
    pub struct CommandExecutor {}

    impl qobject::CommandExecutor {
        #[qinvokable]
        pub fn exit(&self) {
            println!("[INFO] Exiting.");
        }
        #[qinvokable]
        pub fn run_in_background(&self) {
            println!("[INFO] Running in background and minimizing to tray if supported.");
        }
        #[qinvokable]
        pub fn test_fn(&self) {
            println!("[DEBUG] test_fn");
        }
        #[qinvokable]
        pub fn test_signal(&self) {
            signals::test_signal();
        }
    }
}
