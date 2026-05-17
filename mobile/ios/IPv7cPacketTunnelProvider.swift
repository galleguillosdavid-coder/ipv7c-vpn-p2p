import NetworkExtension
import CryptoKit
import os.log

/**
 * IPv7C Network Extension para iOS/macOS (Fase 2: Conquista Móvil - Apple)
 *
 * Usa NetworkExtension / NEPacketTunnelProvider para capturar tráfico
 * a nivel sistema sin requerir jailbreak.
 * Activa perfil `intermittent` para hibernación inteligente en batería.
 */
class IPv7cPacketTunnelProvider: NEPacketTunnelProvider {
    
    private let logger = Logger(subsystem: "com.ipv7c.node", category: "VPN")
    
    /// Punto de entrada: se llama cuando el usuario activa la VPN
    override func startTunnel(options: [String: NSObject]? = nil) async throws {
        logger.info("IPv7C Nodo Soberano - Iniciando en iOS/macOS...")
        
        // Configurar la interfaz TUN virtual
        let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.7.0.255")
        
        let ipv4Settings = NEIPv4Settings(
            addresses: ["10.7.0.2"],
            subnetMasks: ["255.255.255.0"]
        )
        ipv4Settings.includedRoutes = [NEIPv4Route.default()]
        settings.ipv4Settings = ipv4Settings
        
        let dnsSettings = NEDNSSettings(servers: ["10.7.0.53"])
        settings.dnsSettings = dnsSettings
        settings.mtu = 1400
        
        try await setTunnelNetworkSettings(settings)
        logger.info("Túnel TUN establecido. Iniciando lectura de paquetes...")
        
        // Iniciar loop de procesamiento de paquetes
        await processPackets()
    }
    
    /// Loop de procesamiento de paquetes capturados
    private func processPackets() async {
        guard let packetFlow = self.packetFlow as? NEPacketTunnelFlow else { return }
        
        while true {
            let packets = await packetFlow.readPacketObjects()
            for packet in packets {
                // TODO: Pasar paquete al núcleo ipv7c_core via Swift-Rust FFI (UniFFI)
                // Por ahora: re-inyectar el paquete como pass-through
                await packetFlow.writePacketObjects([packet])
            }
        }
    }
    
    override func stopTunnel(with reason: NEProviderStopReason) async {
        logger.info("IPv7C detenido. Razón: \(reason.rawValue)")
    }
}
