package com.plugin.accessibility

import android.app.Activity
import android.view.accessibility.AccessibilityEvent
import android.accessibilityservice.AccessibilityService
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class Accessibility(private val activity: Activity) : Plugin(activity) {
    val accessibilityService: AccessibilityService =
            object : AccessibilityService() {
                override fun onAccessibilityEvent(event: AccessibilityEvent?) {
                    event?.let {
                        // Handle accessibility events here
                        println("Accessibility Event: ${event.eventType}")
                    }
                }

                override fun onInterrupt() {
                    // Handle service interruptions here
                    println("Accessibility Service interrupted")
                }
            }

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    override fun load(webView: WebView) {
        println("Loaded")
    }
}
