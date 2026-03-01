package com.batmonic.app

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    // Register USB audio plugin before super.onCreate (which initializes the WebView)
    pluginManager.load(null, "usb-audio", UsbAudioPlugin(this), "{}")
    super.onCreate(savedInstanceState)
  }
}
