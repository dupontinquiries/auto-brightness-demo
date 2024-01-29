pub mod settingswindow;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

use std::sync::{Arc, Mutex};

use crate::daemon::{Daemon};

pub fn launch_window(daemon: Arc<Mutex<Daemon>>) {
    let gui_thread = std::thread::spawn(move || {
        // Create the application and engine
        let mut app = QGuiApplication::new();

        let mut engine = QQmlApplicationEngine::new();

        // Load the QML path into the engine
        if let Some(engine) = engine.as_mut() {
            engine.load(&QUrl::from("qml/settingswindow.qml"));
        }

        // Start the app
        if let Some(app) = app.as_mut() {
            app.exec();
        }
    });
    let _ = gui_thread.join();
}
