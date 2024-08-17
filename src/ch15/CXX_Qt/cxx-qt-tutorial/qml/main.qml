// Qt套件
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// CXX-Qt套件
import com.kdab.cxx_qt.demo 1.0

// 視窗
Window {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640

    // 對應 QObject
    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    // 控制項
    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(myObject.number)
        }

        Label {
            text: qsTr("String: %1").arg(myObject.string)
        }

        Button {
            text: qsTr("Increment Number")
            // 呼叫 Rust的方法
            onClicked: myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")
            // 呼叫 Rust的方法
            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }
    }
}
