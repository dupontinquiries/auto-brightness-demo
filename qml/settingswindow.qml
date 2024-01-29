import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick 2.12
// import QtGraphicalEffects 1.12

import brightcast_signals 1.0

Window {
    id: window
    title: qsTr("Bright Cast Settings")
    visible: true
    height: 480
    width: 640
    color: "#111111"
    // color: "#11111100"
    // color: "rgba(24, 24, 24, 24)"


    CommandExecutor {
        id: command_executor
    }

    // FastBlur {
    //     id: blurEffect
    //     anchors.fill: window
    //     source: window
    //     radius: 8
    //     // samples: 16
    // }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter

        spacing: 10

        Button {
            text: "Quit Bright Cast"
            // onClicked: command_executor.exit()
        }
        Button {
            text: "Close window and run in background."
            // onClicked: command_executor.runInBackground()
        }
        Button {
            text: "Close window and run in background."
            // onClicked: command_executor.testFn()
            onClicked: command_executor.testFn()
        }
    }
}
