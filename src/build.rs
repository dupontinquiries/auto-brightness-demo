// build.rs

use cxx_qt_build::CxxQtBuilder;

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "com.impakt.brightcast.settingswindow",
            rust_files: &["src/gui/settingswindow.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}

// fn main() {
//     CxxQtBuilder::new()
//         // Link Qt's Network library
//         // - Qt Core is always linked
//         // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
//         // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
//         // - Qt Qml requires linking Qt Network on macOS
//         .qt_module("Network")
//         .qt_module("QtGraphicalEffects")
//         .qt_module("QtQuick")
//         // Generate C++ from the `#[cxx_qt::bridge]` module
//         .file("src/gui/settingswindow/mod.rs")
//         // Generate C++ code from the .qrc file with the rcc tool
//         // https://doc.qt.io/qt-6/resources.html
//         .qrc("src/qml/settingswindow.qrc")
//         .setup_linker()
//         .build();
// }
