# Estrategia de Auto-Expansión y Control de Hardware (La Red del Futuro)

Este documento define la arquitectura y el plan de desarrollo para que **IPv7 Sovereign Mesh** deje de ser una red superpuesta dependiente (overlay) y se convierta en un **organismo autónomo capaz de manipular hardware, puentear conexiones físicas y auto-expandirse sin internet**.

## 1. El Problema de la Dependencia IP
Actualmente, IPv7C requiere una capa de internet subyacente proporcionada por el sistema operativo. Si Windows o Android pierden la conexión Wi-Fi, el nodo queda aislado. La "Red del Futuro" no espera a tener internet; **crea su propio internet**.

## 2. El Módulo `HardwareController` (Controlador Físico)
El próximo gran salto evolutivo es construir un módulo con permisos de sistema (Root/Admin) capaz de interactuar directamente con los chips de radio de los dispositivos.

### Fase 1: Enlaces de Corto Alcance y Supervivencia
Cuando un nodo detecta pérdida total de conectividad IP, activa protocolos de escaneo físico:
*   **Bluetooth Low Energy (BLE):** Los dispositivos emiten balizas (beacons) indetectables anunciando su clave pública. Si dos celulares con IPv7C se acercan, forman una micro-malla.
*   **Wi-Fi Direct / Ad-Hoc:** Las computadoras y celulares escanean y se conectan entre sí directamente, sin necesidad de un router físico o punto de acceso tradicional.

### Fase 2: Puenteo Solidario (Hotspot Autónomo)
El verdadero poder de la malla. Si el dispositivo **A** (Laptop en un bosque) está aislado, pero el dispositivo **B** (Celular a 20 metros) tiene señal 5G o satelital:
*   **A** y **B** se descubren vía BLE o Wi-Fi Direct.
*   El nodo de **B** levanta automáticamente un puente seguro (Soft-AP / Hotspot) compartiendo su ancho de banda con **A**.
*   Todo ocurre de forma transparente, cifrada y gobernada por la Sovereign AI.

## 3. Omnipresencia Absoluta: Todos los Dispositivos son Nodos

La malla está diseñada para asimilar y utilizar cualquier dispositivo electrónico que posea un microprocesador y un radio:

### 3.1. Routers Caseros (OpenWrt / pfSense)
El software se instala a nivel kernel en el router de la casa. En lugar de conectar dispositivos a una VPN, toda la casa entera entra en la Malla Soberana de forma predeterminada.
### 3.2. Centros de Datos y Backbone (Super-Relays)
Servidores en la nube de alto rendimiento operando con eBPF/XDP. Funcionan como autopistas troncales capaces de enrutar millones de paquetes por segundo para unir continentes cuando las conexiones P2P locales son imposibles.
### 3.3. Satélites (Starlink, LoRaWAN)
Nodos equipados con antenas direccionales o modems satelitales que funcionan como "Gateways de Supervivencia". Si se cae el internet submarino, los nodos saltan a constelaciones LEO (Starlink) o redes de radiofrecuencia de largo alcance (LoRa) para mantener viva la malla global.
### 3.4. Enjambre IoT (Saltos de Rana)
Asimilación de impresoras inteligentes, Smart TVs, bombillas Wi-Fi y refrigeradores. Estos dispositivos rara vez inician conexiones, pero la Sovereign AI los utiliza como **relés pasivos** (repetidores) para que un paquete de datos "salte" por toda la casa hasta llegar al destino.

### 3.5. Transmisión Multi-Medio (Más allá de la Radio)
La malla no se limita a frecuencias de radio. Para ser verdaderamente omnipresente e indestructible, el nodo es capaz de modular y transmitir datos sobre *cualquier medio físico* disponible:
*   **Medios Ópticos (Luz y Láser):** Compatibilidad con troncales de fibra óptica, así como **Li-Fi** (transmisión de datos a través del parpadeo imperceptible de bombillas LED) y enlaces Láser de espacio libre (FSO) para cruzar ciudades sin cables.
*   **Infraestructura Eléctrica (Powerline / PLC):** Uso de la red eléctrica de la ciudad o el hogar. Al enchufar un dispositivo a la pared, todo el cableado eléctrico de cobre del edificio se convierte instantáneamente en el medio de transmisión de la malla.
*   **Cobre y Cables Físicos:** Aprovechamiento de conexiones LAN directas, cables Ethernet, coaxial y DSL, formando mallas físicas blindadas contra la interferencia de radio o inhibidores de señal.
*   **Ondas Acústicas (Ultrasonido):** En escenarios de extrema censura o aislamiento tipo "búnker", el software utilizará los altavoces y micrófonos de los dispositivos para transmitir pequeños paquetes de datos mediante ultrasonidos inaudibles para el oído humano.

## 4. Evolución de la Sovereign AI
Para orquestar este caos, la IA del nodo ya no solo analiza latencia TCP/UDP, sino que:
*   **Gestiona energía:** Decide no encender Wi-Fi Direct si el celular tiene menos del 15% de batería, prefiriendo BLE.
*   **Optimiza el Espectro:** Cambia de canales de radio para evitar interferencias.
*   **Toma de Decisiones Soberanas:** Evalúa si es más rápido enviar un paquete a través del celular 5G del vecino, o esperar a rebotarlo por la Smart TV del pasillo.

---
**Estado:** ✅ **IMPLEMENTADO** — `hardware_controller.py` creado con 8 adaptadores físicos (BLE, Wi-Fi Direct, LAN, Serial, UPnP/IoT, Ultrasonido, Li-Fi, PLC). Integrado en el núcleo `P2PVpnNode` con monitoreo activo de aislamiento. Ver [PLAN_EJECUCION_HARDWARE.md](PLAN_EJECUCION_HARDWARE.md) para el detalle completo.
