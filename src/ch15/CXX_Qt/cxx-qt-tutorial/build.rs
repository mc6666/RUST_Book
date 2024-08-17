use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network") // 必須引用Network模組
        .qml_module(QmlModule {
            uri: "com.kdab.cxx_qt.demo",          // 必須指定URI，提供main使用
            rust_files: &["src/cxxqt_object.rs"], // 指定QObject檔案
            qml_files: &["qml/main.qml"],         // 指定QML檔案
            ..Default::default()
        })
        .build();
}