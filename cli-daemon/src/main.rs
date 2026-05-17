use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("IPv7C Daemon (ipv7cd) Iniciando...");
    println!("Cargando core nativo para enrutamiento P2P...");
    
    // Inicializar el nodo P2P desde el core nativo en el puerto 8765
    let p2p_node = ipv7c_core::p2p::P2pNode::new(8765).await?;
    
    println!("Demonio activo. Escuchando en modo Gateway UDP en el puerto 8765...");
    
    // Ejecutar el bucle de enrutamiento principal
    p2p_node.routing_loop().await;
    
    Ok(())
}
