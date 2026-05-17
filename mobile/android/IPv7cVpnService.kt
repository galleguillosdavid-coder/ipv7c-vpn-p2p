package com.ipv7c.android

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.net.VpnService
import android.os.ParcelFileDescriptor
import android.util.Log

/**
 * IPv7C VPN Service para Android (Fase 2: Conquista Móvil)
 * 
 * Se conecta al VpnService nativo del SO para capturar tráfico sin root.
 * Activa el perfil `intermittent` del núcleo para conservar batería.
 */
class IPv7cVpnService : VpnService() {

    companion object {
        const val TAG = "IPv7C-VPN"
        const val CHANNEL_ID = "ipv7c_vpn_channel"
        const val NOTIFICATION_ID = 1
    }

    private var vpnInterface: ParcelFileDescriptor? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.i(TAG, "IPv7C Nodo Soberano - Iniciando en Android...")
        startForegroundNotification()
        establishVpnTunnel()
        return Service.START_STICKY
    }

    /**
     * Establece el túnel TUN mediante VpnService.Builder
     * En producción, aquí se enlaza con libipv7c_core.so via JNI
     */
    private fun establishVpnTunnel() {
        val builder = Builder()
            .setSession("IPv7C-Node")
            .addAddress("10.7.0.1", 24)         // Dirección virtual IPv7C
            .addRoute("0.0.0.0", 0)             // Captura todo el tráfico
            .addDnsServer("10.7.0.53")          // DNS soberano dentro de la malla
            .setMtu(1400)                        // MTU optimizado para malla P2P
            .setBlocking(false)

        vpnInterface = builder.establish()
        Log.i(TAG, "Túnel VPN establecido: ${vpnInterface?.fd}")
        
        // TODO: Lanzar ipv7c_core (Rust/JNI) para procesar paquetes del fd
    }

    private fun startForegroundNotification() {
        val channel = NotificationChannel(
            CHANNEL_ID, "IPv7C Nodo Activo",
            NotificationManager.IMPORTANCE_LOW
        )
        getSystemService(NotificationManager::class.java)?.createNotificationChannel(channel)
        
        val notification = Notification.Builder(this, CHANNEL_ID)
            .setContentTitle("IPv7C Soberano")
            .setContentText("Malla P2P activa. Modo: intermittent")
            .setSmallIcon(android.R.drawable.ic_lock_lock)
            .build()
        
        startForeground(NOTIFICATION_ID, notification)
    }

    override fun onDestroy() {
        super.onDestroy()
        vpnInterface?.close()
        Log.i(TAG, "Nodo IPv7C detenido.")
    }
}
