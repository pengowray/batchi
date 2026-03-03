package com.batmonic.app

import android.os.Bundle
import android.util.Log
import android.webkit.PermissionRequest
import android.webkit.WebChromeClient
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge

private const val TAG = "MainActivity"

class MainActivity : TauriActivity() {
  private var usbAudioPlugin: UsbAudioPlugin? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    // Register USB audio plugin before super.onCreate (which initializes the WebView)
    val plugin = UsbAudioPlugin(this)
    usbAudioPlugin = plugin
    pluginManager.load(null, "usb-audio", plugin, "{}")
    super.onCreate(savedInstanceState)

    // Defer WebView permission setup until the view hierarchy is ready.
    // Using post{} ensures the WebView is findable in the view tree.
    window.decorView.post {
      setupWebViewPermissions()
    }
  }

  @Suppress("DEPRECATION")
  override fun onRequestPermissionsResult(
    requestCode: Int,
    permissions: Array<out String>,
    grantResults: IntArray
  ) {
    Log.i(TAG, "onRequestPermissionsResult: code=$requestCode, results=${grantResults.toList()}")
    super.onRequestPermissionsResult(requestCode, permissions, grantResults)
    // Forward to USB audio plugin for RECORD_AUDIO permission handling
    usbAudioPlugin?.handlePermissionResult(requestCode, grantResults)
  }

  private fun setupWebViewPermissions() {
    // Find the WebView created by Tauri and override its WebChromeClient
    // to grant RESOURCE_AUDIO_CAPTURE requests from the frontend JavaScript.
    val rootView = window.decorView
    val webView = findWebView(rootView)
    if (webView == null) {
      Log.w(TAG, "setupWebViewPermissions: WebView not found in view hierarchy")
      return
    }
    Log.i(TAG, "setupWebViewPermissions: Found WebView, setting up WebChromeClient")
    val originalClient = webView.webChromeClient
    webView.webChromeClient = object : WebChromeClient() {
      override fun onPermissionRequest(request: PermissionRequest) {
        Log.i(TAG, "onPermissionRequest: ${request.resources.toList()}")
        val resources = request.resources
        if (resources.contains(PermissionRequest.RESOURCE_AUDIO_CAPTURE)) {
          Log.i(TAG, "Granting RESOURCE_AUDIO_CAPTURE")
          request.grant(resources)
        } else {
          originalClient?.onPermissionRequest(request) ?: request.deny()
        }
      }
    }
  }

  private fun findWebView(view: android.view.View): WebView? {
    if (view is WebView) return view
    if (view is android.view.ViewGroup) {
      for (i in 0 until view.childCount) {
        findWebView(view.getChildAt(i))?.let { return it }
      }
    }
    return null
  }
}
